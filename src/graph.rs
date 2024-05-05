// IMPORT STANDARD LIBRARY MODULES FOR FILE I/O, COLLECTIONS, AND MORE
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use csv::ReaderBuilder;
use std::fmt::Write as FmtWrite;

// STRUCT TO HOLD COMPANY DETAILS
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Company {
    pub name: String,
    pub domain_name: String,
}

// ENUM TO REPRESENT DIFFERENT TYPES OF BUSINESS RELATIONSHIPS OR EDGES
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeTypes {
    Customer,
    Supplier,
    Investor,
    Investee,
    Partner,
    Competitor,
    None,
}

// STRUCT TO HOLD CONNECTION DETAILS BETWEEN TWO COMPANIES
#[derive(Debug, Clone)]
pub struct CompanyConnection {
    pub id: String,
    pub tail: Company,
    pub head: Company,
    pub edge: EdgeTypes,
    pub update_time: String,
}

// IMPLEMENTATION BLOCK FOR `CompanyConnection`
impl CompanyConnection {
    // METHOD TO CREATE A RECIPROCAL CONNECTION BASED ON THE CURRENT CONNECTION
    pub fn reciprocal_connection(&self) -> CompanyConnection {
        CompanyConnection {
            id: format!("{}-RECI", self.id),
            tail: self.head.clone(),
            head: self.tail.clone(),
            edge: match self.edge {
                EdgeTypes::Customer => EdgeTypes::Supplier,
                EdgeTypes::Supplier => EdgeTypes::Customer,
                EdgeTypes::Investor => EdgeTypes::Investee,
                EdgeTypes::Investee => EdgeTypes::Investor,
                EdgeTypes::Partner | EdgeTypes::Competitor | EdgeTypes::None => self.edge,
            },
            update_time: self.update_time.clone(),
        }
    }
}

// FUNCTION TO MATCH A STRING INTO AN `EdgeTypes` ENUM
fn match_EdgeTypes(s: &str) -> EdgeTypes {
    match s.to_lowercase().as_str() {
        "customer" => EdgeTypes::Customer,
        "supplier" => EdgeTypes::Supplier,
        "investment" => EdgeTypes::Investor,
        "investee" => EdgeTypes::Investee,
        "partnership" => EdgeTypes::Partner,
        "competitor" => EdgeTypes::Competitor,
        _ => EdgeTypes::None,
    }
}

// TYPE ALIAS FOR A HASHMAP THAT HOLDS AN ADJACENCY LIST REPRESENTATION OF A GRAPH
pub type AdjacencyList = HashMap<Company, Vec<CompanyConnection>>;

// FUNCTION TO CREATE A GRAPH FROM A CSV FILE
pub fn create_graph(file_path: &str) -> AdjacencyList {
    let file = File::open(file_path).unwrap();
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut graph: AdjacencyList = HashMap::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let parts = record.iter().collect::<Vec<_>>();

        // SKIP RECORD IF IT HAS FEWER THAN 8 FIELDS OR CONTAINS "N/A"
        if parts.len() < 8 || parts.iter().any(|&part| part == "N/A") {
            continue;
        }

        let edge_type = match_EdgeTypes(parts[3]);
        // SKIP RECORD IF EDGE TYPE IS 'None'
        if edge_type == EdgeTypes::None {
            continue;
        }
 
        let company_connection = CompanyConnection {
            id: parts[0].to_string(),
            tail: Company {
                name: parts[1].to_string(),
                domain_name: parts[5].to_string(),
            },

            head: Company {
                name: parts[2].to_string(),
                domain_name: parts[6].to_string(),
            },

            edge: edge_type,
            update_time: parts[4].to_string(),
        };

        // ADD CONNECTION AND ITS RECIPROCAL TO THE GRAPH
        graph.entry(company_connection.tail.clone()).or_insert_with(Vec::new).push(company_connection.clone());
        graph.entry(company_connection.head.clone()).or_insert_with(Vec::new).push(company_connection.reciprocal_connection());
    }

    return graph;
}

// FUNCTION TO PRINT GRAPH CONTENT TO A FILE
pub fn write_graph(graph: &AdjacencyList, path: &str) {
    let mut file = File::create(path).unwrap();
    let mut content = String::new();
    for (company, connections) in graph {
        writeln!(content, "Company: {}, Domain: {}", company.name, company.domain_name).unwrap();
        for connection in connections {
            writeln!(content, "   Connection ID: {}, Tail Node: {:?}, Head Node: {:?}, Edge Type: {:?}, Update Time: {}", 
                     connection.id, connection.tail, connection.head, connection.edge, connection.update_time).unwrap();
        }
    }
    file.write_all(content.as_bytes()).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    // DEFINE A TEST FUNCTION TO CHECK THE CORRECTNESS OF THE RECIPROCAL CONNECTION CREATION
    #[test]
    fn test_reciprocal_connection() {
        
        // CREATE AN ORIGINAL COMPANY CONNECTION
        let original_connection = CompanyConnection {
            id: "123".to_string(),
            tail: Company { name: "CompanyA".to_string(), domain_name: "companya.com".to_string() },
            head: Company { name: "CompanyB".to_string(), domain_name: "companyb.com".to_string() },
            edge: EdgeTypes::Customer,
            update_time: "2021-04-01".to_string(),
        };
        
        // GENERATE A RECIPROCAL CONNECTION FROM THE ORIGINAL
        let reciprocal = original_connection.reciprocal_connection();

        // CHECK ALL PARTS OF `CompanyConnection` AND ITS RECIPROCAL
        assert_eq!(reciprocal.tail.name, original_connection.head.name);
        assert_eq!(reciprocal.head.name, original_connection.tail.name);
        assert_eq!(reciprocal.edge, EdgeTypes::Supplier);
        assert_eq!(reciprocal.id, "123-RECI");
    }

    // DEFINE A TEST FUNCTION TO VERIFY THE EDGE TYPE PARSING FUNCTIONALITY
    #[test]
    fn test_match_EdgeTypes() {

        // FOR EACH VALUE OF POSSIBLE VALUE OF `type` IN `links.csv`, CHECK IF THE CORRECT VALUE OF `EdgeTypes` IS PARSED
        assert_eq!(match_EdgeTypes("customer"), EdgeTypes::Customer);
        assert_eq!(match_EdgeTypes("supplier"), EdgeTypes::Supplier);
        assert_eq!(match_EdgeTypes("investment"), EdgeTypes::Investor);
        assert_eq!(match_EdgeTypes("investee"), EdgeTypes::Investee);
        assert_eq!(match_EdgeTypes("partnership"), EdgeTypes::Partner);
        assert_eq!(match_EdgeTypes("competitor"), EdgeTypes::Competitor);
        assert_eq!(match_EdgeTypes("some other value"), EdgeTypes::None);    
    }
}
