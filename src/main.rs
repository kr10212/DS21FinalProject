// IMPORT MODULES FROM OTHER FILES IN THE PROJECT
mod graph;
mod distribution;

// IMPORT NECESSARY RUST STANDARD LIBRARY AND CUSTOM MODULE FUNCTIONALITIES
use graph::{EdgeTypes, create_graph, write_graph};
use distribution::write_degree_distribution;
use std::process::Command;

fn main() {
    // DECLARE FILENAMES FOR INPUT AND OUTPUT
    let input_filename = "links.csv";
    let output_filename = "graph_links.txt";

    // CREATE A GRAPH FROM THE INPUT FILE
    let graph = create_graph(&input_filename);
    println!("Graph has been created.");
    
    // PRINT THE GRAPH TO A TEXT FILE
    write_graph(&graph, output_filename);
    println!("Graph has been written to {}. \nMoving forward with distribution analysis. This takes about a minute.", output_filename);
    println!("While you wait, please note that you must close the current `matplotlib` window before viewing the next one.`matplotlib` can only display one window at a time.");

    // GENERATE DISTRIBUTION DATA FOR FIRST AND SECOND DEGREE CONNECTIONS
    write_degree_distribution("Distance_1_Distribution.txt", &graph, 1, vec![EdgeTypes::Customer, EdgeTypes::Supplier, EdgeTypes::Investor, EdgeTypes::Investee, EdgeTypes::Partner, EdgeTypes::Competitor].into());
    write_degree_distribution("Distance_2_Distribution.txt", &graph, 2, vec![EdgeTypes::Customer, EdgeTypes::Supplier, EdgeTypes::Investor, EdgeTypes::Investee, EdgeTypes::Partner, EdgeTypes::Competitor].into());
    
    // EXECUTE PYTHON SCRIPTS TO ANALYZE THE DISTRIBUTION DATA USING EXTERNAL COMMAND
    let deg1_analysis = Command::new("python3").arg("src/analysis.py").arg("Distance_1_Distribution.txt").output().expect("Failed to execute command");
    let deg2_analysis = Command::new("python3").arg("src/analysis.py").arg("Distance_2_Distribution.txt").output().expect("Failed to execute command");
    
    // PRINT OUTPUT FROM PYTHON SCRIPT ANALYSIS TO THE CONSOLE
    println!("Distribution calculations are complete. Close the `matplotlib` window to output. ");
    print!("\nAnalysis for Distance = 1 Distribution:\n{}", String::from_utf8_lossy(&deg1_analysis.stdout));
    print!("\nAnalysis for Distance = 2 Distribution:\n{}", String::from_utf8_lossy(&deg2_analysis.stdout));
}
