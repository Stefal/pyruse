use crate::domain::{Config, Module, ModulesPort, Record, Step};
use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug)]
struct Node {
  name: String,
  module: Module,
  then_dest: isize, // <0 for a leaf-Node
  else_dest: isize, // <0 for a leaf-Node
}

impl Node {
  fn run(&self, record: &mut Record) -> isize {
    if let Ok(b) = self.module.run(record) {
      if b {
        self.then_dest
      } else {
        self.else_dest
      }
    } else {
      -1
    }
  }
}

pub struct Workflow {
  nodes: Vec<Node>,
}

impl Workflow {
  pub fn run(&self, record: &mut Record) {
    let mut i = 0 as isize;
    while {
      i = self.nodes[i as usize].run(record);
      i >= 0
    } {}
  }

  pub fn build(conf: &mut Config, available: &dyn ModulesPort) -> Workflow {
    let mut seen: Vec<String> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();
    let mut dangling: DanglingInfo = HashMap::new();
    for (name, chain) in conf.actions.drain(..) {
      build_chain(name, chain, &mut nodes, &mut seen, &mut dangling, available);
    }
    if nodes.is_empty() {
      panic!("A configuration must have at least one module.");
    }
    if !dangling.is_empty() {
      let mut error = "Incomplete configuration:".to_string();
      let mut unknown = false;
      for (o, v) in dangling {
        if let Some(c) = o {
          unknown = true;
          error = format!(
            "{}\n\tReference to unknown chain “{}” found at:",
            &error, &c
          );
          for (i, is_then) in v {
            error = format!(
              "{}\n\t  {}:{}",
              &error,
              nodes[i].name,
              if is_then { "then" } else { "else" }
            );
          }
        }
      }
      if unknown {
        panic!(error);
      }
    }
    Workflow { nodes }
  }
}

type DanglingInfo = HashMap<Option<String>, Vec<(usize, bool)>>;

fn build_chain(
  chain_name: String,
  chain: Vec<Step>,
  nodes: &mut Vec<Node>,
  seen: &mut Vec<String>,
  dangling: &mut DanglingInfo,
  available: &dyn ModulesPort,
) {
  let mut index = 0;
  for step in chain {
    let next_node_u = nodes.len();
    let next_node_i = next_node_u as isize;
    let mut then_was_used = false;
    if index == 0 {
      if let Some(v) = dangling.remove(&Some(&chain_name).cloned()) {
        for (i, is_then) in v {
          match is_then {
            true => nodes.get_mut(i).unwrap().then_dest = next_node_i,
            false => nodes.get_mut(i).unwrap().else_dest = next_node_i,
          };
        }
      } else if let Some(v) = dangling.remove(&None) {
        for (i, is_then) in v {
          match is_then {
            true => nodes.get_mut(i).unwrap().then_dest = next_node_i,
            false => nodes.get_mut(i).unwrap().else_dest = next_node_i,
          };
        }
      }
    } else {
      nodes.get_mut(next_node_u - 1).unwrap().then_dest = next_node_i;
    }
    let name = chain_name
      .clone()
      .add(&format!("[{}]:{}", index, &step.module));
    let module = Module::new(step.module, step.args, available)
      .expect(&format!("Module {} could not be created.", &name));
    let then_dest = step
      .then_dest
      .map(|c| {
        if seen.contains(&c) {
          panic!(format!("Configuration loop at {}:then", name));
        }
        then_was_used = true;
        node_wants_chain(next_node_u, true, dangling, Some(c));
        -1
      })
      .unwrap_or(-1);
    let else_dest = step
      .else_dest
      .map(|c| {
        if seen.contains(&c) {
          panic!(format!("Configuration loop at {}:else", name));
        }
        node_wants_chain(next_node_u, false, dangling, Some(c));
        -1
      })
      .unwrap_or({
        node_wants_chain(next_node_u, false, dangling, None);
        -1
      });
    nodes.push(Node {
      name,
      module,
      then_dest,
      else_dest,
    });
    index += 1;
    if then_was_used {
      break;
    }
  }
}

fn node_wants_chain(
  next_node_index: usize,
  is_then: bool,
  dangling: &mut DanglingInfo,
  wanted_chain: Option<String>,
) {
  let mut d = dangling.remove(&wanted_chain).unwrap_or(Vec::new());
  d.push((next_node_index, is_then));
  dangling.insert(wanted_chain, d);
}

#[cfg(test)]
mod tests {
  use crate::domain::test_util::*;
  use crate::domain::{AvailableAction, AvailableFilter, Chain, Config, Record, Step, Value, Workflow};
  use indexmap::IndexMap;
  use std::collections::HashMap;

  #[test]
  #[should_panic(expected = "A configuration must have at least one module.")]
  fn empty_config_results_in_panic() {
    // Given
    let mut conf = Config {
      actions: IndexMap::new(),
      options: HashMap::new(),
    };
    let mods = FakeModulesAdapter::new(&[], &[]);

    // When
    let _wf = Workflow::build(&mut conf, &mods);
  }

  #[test]
  fn config_with_one_module_is_ok() {
    // Given
    let mut actions: IndexMap<String, Chain> = IndexMap::new();
    actions.insert(
      "chain1".to_string(),
      vec![Step {
        module: ACT_NAME.to_string(),
        args: HashMap::new(),
        then_dest: None,
        else_dest: None,
      }],
    );
    let mut conf = Config {
      actions,
      options: HashMap::new(),
    };
    let aa = [AvailableAction {
      name: ACT_NAME.to_string(),
      cons: |_| Box::new(FakeAction {}),
    }];
    let mods = FakeModulesAdapter::new(&aa, &[]);
    let mut record: Record = HashMap::new();

    // When
    let wf = Workflow::build(&mut conf, &mods);
    wf.run(&mut record);

    // Then
    assert!(wf.nodes.len() == 1);
    assert!(wf.nodes[0].name == "chain1[0]:fake_action");
    assert!(wf.nodes[0].then_dest < 0);
    assert!(wf.nodes[0].else_dest < 0);
    assert!(record[ACT_NAME] == Value::Int(1));
    assert!(record.get(FLT_NAME) == None);
  }

  #[test]
  fn explicit_chain_to_chain_link_works() {
    // Given
    let mut actions: IndexMap<String, Chain> = IndexMap::new();
    actions.insert(
      "chain1".to_string(),
      vec![Step {
        module: FLT_NAME.to_string(),
        args: HashMap::new(),
        then_dest: None,
        else_dest: Some("chain2".to_string()),
      }],
    );
    actions.insert(
      "chain2".to_string(),
      vec![Step {
        module: ACT_NAME.to_string(),
        args: HashMap::new(),
        then_dest: None,
        else_dest: None,
      }],
    );
    let mut conf = Config {
      actions,
      options: HashMap::new(),
    };
    let aa = [AvailableAction {
      name: ACT_NAME.to_string(),
      cons: |_| Box::new(FakeAction {}),
    }];
    let af = [AvailableFilter {
      name: FLT_NAME.to_string(),
      cons: |_| Box::new(FakeFilter {}),
    }];
    let mods = FakeModulesAdapter::new(&aa, &af);
    let mut record: Record = HashMap::new();

    // When
    let wf = Workflow::build(&mut conf, &mods);
    wf.run(&mut record);

    // Then
    assert!(wf.nodes.len() == 2);
    assert!(wf.nodes[0].name == "chain1[0]:fake_filter");
    assert!(wf.nodes[1].name == "chain2[0]:fake_action");
    assert!(wf.nodes[0].then_dest < 0);
    assert!(wf.nodes[0].else_dest == 1);
    assert!(wf.nodes[1].then_dest < 0);
    assert!(wf.nodes[1].else_dest < 0);
    assert!(record[ACT_NAME] == Value::Int(1));
    assert!(record[FLT_NAME] == Value::Int(1));
  }

  #[test]
  fn implicit_chain_to_chain_link_works() {
    // Given
    let mut actions: IndexMap<String, Chain> = IndexMap::new();
    actions.insert(
      "chain1".to_string(),
      vec![Step {
        module: FLT_NAME.to_string(),
        args: HashMap::new(),
        then_dest: None,
        else_dest: None,
      }],
    );
    actions.insert(
      "chain2".to_string(),
      vec![Step {
        module: ACT_NAME.to_string(),
        args: HashMap::new(),
        then_dest: None,
        else_dest: None,
      }],
    );
    let mut conf = Config {
      actions,
      options: HashMap::new(),
    };
    let aa = [AvailableAction {
      name: ACT_NAME.to_string(),
      cons: |_| Box::new(FakeAction {}),
    }];
    let af = [AvailableFilter {
      name: FLT_NAME.to_string(),
      cons: |_| Box::new(FakeFilter {}),
    }];
    let mods = FakeModulesAdapter::new(&aa, &af);
    let mut record: Record = HashMap::new();

    // When
    let wf = Workflow::build(&mut conf, &mods);
    wf.run(&mut record);

    // Then
    assert!(wf.nodes.len() == 2);
    assert!(wf.nodes[0].name == "chain1[0]:fake_filter");
    assert!(wf.nodes[1].name == "chain2[0]:fake_action");
    assert!(wf.nodes[0].then_dest < 0);
    assert!(wf.nodes[0].else_dest == 1);
    assert!(wf.nodes[1].then_dest < 0);
    assert!(wf.nodes[1].else_dest < 0);
    assert!(record[ACT_NAME] == Value::Int(1));
    assert!(record[FLT_NAME] == Value::Int(1));
  }
}
