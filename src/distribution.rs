// IMPORT REQUIRED STRUCTS AND TRAITS FROM `graph` MODULE AND STANDARD LIBRARIES
use crate::graph::{AdjacencyList, EdgeTypes, Company, CompanyConnection};
use std::vec::Vec;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{Write};
use std::collections::HashMap;
 
// FUNCTION TO COUNT EDGES BASED ON DEGREE AND EDGE TYPES IN A GRAPH
fn count_edges(graph: &AdjacencyList, degrees: &usize, company: &Company, edges: &Option<Vec<EdgeTypes>>) -> usize {
    let mut queue = VecDeque::new();                                                    // INITIALIZE QUEUE FOR BFS
    let mut visited = HashSet::new();                                                   // SET TO KEEP TRACK OF VISITED COMPANIES
    let mut num_edges: usize = 0;                                                       // COUNTER FOR NUMBER OF EDGES

    queue.push_back((company, 0));                                                      // ENQUEUE THE STARTING COMPANY WITH DEGREE 0
    visited.insert(company);                                                            // MARK THE STARTING COMPANY AS VISITED

    // CREATE A SET OF ALLOWED EDGE TYPES IF PROVIDED
    let filter_set: HashSet<EdgeTypes> = edges.as_ref().map_or(HashSet::new(), |e| e.iter().cloned().collect());

    // PROCESS EACH COMPANY IN THE QUEUE
    while let Some((current_company, current_degree)) = queue.pop_front() {
        if current_degree >= *degrees {
            continue;                                                                   // SKIP IF THE CURRENT DEGREE EXCEEDS THE LIMIT
        }

        if let Some(connections) = graph.get(current_company) {
            for connection in connections {
                // CHECK IF EDGE TYPE IS ALLOWED
                if edges.is_none() || filter_set.contains(&connection.edge) {
                    if !visited.contains(&connection.head) {
                        visited.insert(&connection.head);                               // MARK AS VISITED
                        queue.push_back((&connection.head, current_degree + 1));        // ENQUEUE WITH INCREMENTED DEGREE
                        num_edges += 1;                                                 // INCREMENT EDGE COUNT
                    }
                }
            }
        }
    }

    return num_edges;                                                                   // RETURN THE TOTAL NUMBER OF EDGES FOUND
}

// FUNCTION TO FIND THE DISTRIBUTION OF EDGES UP TO A CERTAIN DEGREE FOR EACH COMPANY IN A GRAPH
fn find_degree_distribution(graph: &AdjacencyList, degrees: &usize, edges: &Option<Vec<EdgeTypes>>) -> Vec<usize> {
    
    // INITIALIZE DEFAULT EDGE TYPES IF NONE PROVIDED
    edges.clone().unwrap_or(vec![EdgeTypes::Customer, EdgeTypes::Supplier, EdgeTypes::Investor, EdgeTypes::Investee, EdgeTypes::Partner, EdgeTypes::Competitor, EdgeTypes::None]);
    let mut distribution: Vec<usize> = Vec::new();                                      // VECTOR TO STORE EDGE COUNT PER DEGREE

    // ITERATE OVER ALL COMPANIES IN THE GRAPH
    for (company, _) in graph {
        let count = count_edges(&graph, &degrees, &company, &edges);                    // COUNT EDGES FOR EACH COMPANY
        if count >= distribution.len() {
            distribution.resize(count + 1, 0);                                          // RESIZE DISTRIBUTION VECTOR IF NEEDED
        }

        distribution[count] +=  1;                                                      // INCREMENT THE COUNT FOR THE DEGREE
    }

    return distribution;                                                                // RETURN THE DISTRIBUTION VECTOR
}

// FUNCTION TO WRITE DEGREE DISTRIBUTION DATA TO A FILE
pub fn write_degree_distribution(path: &str, graph: &AdjacencyList, degrees: usize, edges: Option<Vec<EdgeTypes>>) {
    let mut file = File::create(path).unwrap();                                         // CREATE OR OVERWRITE THE FILE AT `path`
    let numbers = find_degree_distribution(&graph, &degrees, &edges);                   // GET THE DEGREE DISTRIBUTION
    
    // ITERATE THROUGH ELEMENTS OF VECTOR, ADD TO FILE
    for number in numbers {
        writeln!(file, "{}", number).unwrap();                                    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // CREATE A SIMPLE MOCK GRAPH FOR TESTING
    fn create_mock_graph() -> AdjacencyList {
        let mut graph = HashMap::new();
        let company_a = Company { name: "CompanyA".to_string(), domain_name: "companya.com".to_string() };
        let company_b = Company { name: "CompanyB".to_string(), domain_name: "companyb.com".to_string() };

        let connection_ab = CompanyConnection {
            id: "1".to_string(),
            tail: company_a.clone(),
            head: company_b.clone(),
            edge: EdgeTypes::Customer,
            update_time: "2021-04-01".to_string(),
        };

        graph.entry(company_a.clone()).or_insert_with(Vec::new).push(connection_ab.clone());
        graph.entry(company_b.clone()).or_insert_with(Vec::new).push(connection_ab.reciprocal_connection());
        graph
    }

    // TEST THE COUNT_EDGES FUNCTION
    #[test]
    fn test_count_edges() {
        let graph = create_mock_graph();
        let company = Company { name: "CompanyA".to_string(), domain_name: "companya.com".to_string() };
        let edges = Some(vec![EdgeTypes::Customer, EdgeTypes::Supplier]);
        let degree_limit = 1; // SETTING DEGREE LIMIT TO 1

        // COUNT EDGES FOR 'company' WITHIN THE SPECIFIED DEGREE AND EDGE TYPES
        let edge_count = count_edges(&graph, &degree_limit, &company, &edges);

        // ASSERT THAT THE EDGE COUNT IS CORRECT (SHOULD BE 1 IN THIS CASE)
        assert_eq!(edge_count, 1, "The edge count should be 1 for one direct connection.");
    }

    // TEST THE FIND_DEGREE_DISTRIBUTION FUNCTION
    #[test]
    fn test_find_degree_distribution() {
        let graph = create_mock_graph();
        let degree_limit = 1; // LOOKING ONLY AT DIRECT CONNECTIONS
        let edges = Some(vec![EdgeTypes::Customer, EdgeTypes::Supplier]);

        // FIND THE DEGREE DISTRIBUTION FOR THE GRAPH
        let distribution = find_degree_distribution(&graph, &degree_limit, &edges);

        // ASSERT THAT THE DISTRIBUTION HAS THE CORRECT COUNTS
        assert_eq!(distribution, vec![0, 2], "There should be two companies each with one connection within one degree.");
    }
}
