use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;

#[wasm_bindgen]
pub struct TwoSAT {
    num_variables: usize,
    adj_list: Vec<Vec<usize>>,
    reverse_adj_list: Vec<Vec<usize>>,
    assignment: Vec<bool>,
}

#[wasm_bindgen]
impl TwoSAT {
    #[wasm_bindgen(constructor)]
    pub fn new(num_variables: usize) -> Self {
        let adj_list = vec![vec![]; 2 * num_variables];
        let reverse_adj_list = vec![vec![]; 2 * num_variables];
        let assignment = vec![false; num_variables];
        Self {
            num_variables,
            adj_list,
            reverse_adj_list,
            assignment,
        }
    }

    pub fn add_clause(&mut self, x: usize, x_neg: bool, y: usize, y_neg: bool) {
        let x_node = self.var_to_node(x, x_neg);
        let y_node = self.var_to_node(y, y_neg);

        self.adj_list[x_node ^ 1].push(y_node);
        self.adj_list[y_node ^ 1].push(x_node);

        self.reverse_adj_list[y_node].push(x_node ^ 1);
        self.reverse_adj_list[x_node].push(y_node ^ 1);
    }

    pub fn solve(&mut self) -> bool {
        let scc = self.compute_scc();

        for var in 0..self.num_variables {
            if scc[2 * var] == scc[2 * var + 1] {
                return false;
            }
        }

        let mut order = (0..2 * self.num_variables).collect::<Vec<_>>();
        order.sort_by_key(|&v| scc[v]);

        let mut value = vec![None; 2 * self.num_variables];
        for &v in order.iter().rev() {
            if value[v].is_none() {
                value[v] = Some(false);
                value[v ^ 1] = Some(true);
            }
        }

        for var in 0..self.num_variables {
            self.assignment[var] = value[2 * var].unwrap();
        }

        true
    }

    pub fn get_assignment(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.assignment).unwrap()
    }

    fn var_to_node(&self, var: usize, is_negated: bool) -> usize {
        if is_negated {
            2 * var + 1
        } else {
            2 * var
        }
    }

    fn compute_scc(&self) -> Vec<usize> {
        let mut order = vec![];
        let mut visited = vec![false; 2 * self.num_variables];

        fn dfs(v: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
            visited[v] = true;
            for &neighbor in &adj_list[v] {
                if !visited[neighbor] {
                    dfs(neighbor, adj_list, visited, order);
                }
            }
            order.push(v);
        }

        for v in 0..2 * self.num_variables {
            if !visited[v] {
                dfs(v, &self.adj_list, &mut visited, &mut order);
            }
        }

        let mut component = vec![0; 2 * self.num_variables];
        let mut current_component = 0;
        visited.fill(false);

        fn reverse_dfs(v: usize, reverse_adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, component: &mut Vec<usize>, current_component: usize) {
            visited[v] = true;
            component[v] = current_component;
            for &neighbor in &reverse_adj_list[v] {
                if !visited[neighbor] {
                    reverse_dfs(neighbor, reverse_adj_list, visited, component, current_component);
                }
            }
        }

        for &v in order.iter().rev() {
            if !visited[v] {
                reverse_dfs(v, &self.reverse_adj_list, &mut visited, &mut component, current_component);
                current_component += 1;
            }
        }

        component
    }
}

