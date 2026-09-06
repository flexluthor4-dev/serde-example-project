use serde::{Serialize, Deserialize};
use std::fs;

// ============================================
// STEP 1: Define a struct (a data container)
// ============================================
// This is like creating a blueprint for a Person
// #[derive(Serialize, Deserialize)] = "Make this convertible to/from JSON, TOML, YAML"
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
    is_active: bool,
}

// ============================================
// STEP 2: Define a more complex struct
// ============================================
// A Company has multiple Persons (employees)
#[derive(Serialize, Deserialize, Debug)]
struct Company {
    company_name: String,
    employees: Vec<Person>,  // Vec = a list/array
    founded_year: u32,
}

fn main() {
    println!("=== SERDE EXAMPLE PROJECT ===\n");
    
    // ============================================
    // EXAMPLE 1: JSON - Working with JSON strings
    // ============================================
    println!("--- EXAMPLE 1: JSON Serialization ---");
    
    // Create a Person (like filling out a form)
    let alice = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
        is_active: true,
    };
    
    // Convert to JSON (serialize)
    // This turns the Person struct into a JSON string
    let json_string = serde_json::to_string_pretty(&alice).unwrap();
    println!("Person as JSON:\n{}\n", json_string);
    
    // Convert back from JSON (deserialize)
    // This turns a JSON string back into a Person struct
    let alice_from_json: Person = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized back to Person: {:?}\n", alice_from_json);
    
    // ============================================
    // EXAMPLE 2: JSON - Multiple people (arrays)
    // ============================================
    println!("--- EXAMPLE 2: JSON with Arrays ---");
    
    let bob = Person {
        name: "Bob".to_string(),
        age: 28,
        email: "bob@example.com".to_string(),
        is_active: true,
    };
    
    let carol = Person {
        name: "Carol".to_string(),
        age: 35,
        email: "carol@example.com".to_string(),
        is_active: false,
    };
    
    // Create a list of people
    let people = vec![alice.clone(), bob, carol];
    
    // Convert entire list to JSON
    let people_json = serde_json::to_string_pretty(&people).unwrap();
    println!("Multiple people as JSON:\n{}\n", people_json);
    
    // ============================================
    // EXAMPLE 3: Working with a Company
    // ============================================
    println!("--- EXAMPLE 3: Nested Structures ---");
    
    let company = Company {
        company_name: "TechCorp".to_string(),
        employees: people.clone(),
        founded_year: 2015,
    };
    
    let company_json = serde_json::to_string_pretty(&company).unwrap();
    println!("Company as JSON:\n{}\n", company_json);
    
    // ============================================
    // EXAMPLE 4: Reading from a JSON string
    // ============================================
    println!("--- EXAMPLE 4: Parsing JSON ---");
    
    // This is a JSON string (like text from a file or API)
    let json_text = r#"{
        "name": "Diana",
        "age": 32,
        "email": "diana@example.com",
        "is_active": true
    }"#;
    
    // Parse it into a Person struct
    let diana: Person = serde_json::from_str(json_text).unwrap();
    println!("Parsed Diana: {:?}\n", diana);
    
    // ============================================
    // EXAMPLE 5: Save to and read from files
    // ============================================
    println!("--- EXAMPLE 5: File I/O ---");
    
    // Save company to a JSON file
    let company_json_file = serde_json::to_string_pretty(&company).unwrap();
    fs::write("company.json", &company_json_file)
        .expect("Failed to write file");
    println!("✓ Saved company data to company.json");
    
    // Read company back from file
    let file_contents = fs::read_to_string("company.json")
        .expect("Failed to read file");
    let company_from_file: Company = serde_json::from_str(&file_contents).unwrap();
    println!("✓ Read company from file: {}\n", company_from_file.company_name);
    
    // ============================================
    // EXAMPLE 6: TOML (Configuration files)
    // ============================================
    println!("--- EXAMPLE 6: TOML Format ---");
    
    let company_toml = toml::to_string_pretty(&company).unwrap();
    println!("Company as TOML:\n{}\n", company_toml);
    
    // Save to TOML file
    fs::write("company.toml", &company_toml)
        .expect("Failed to write TOML file");
    println!("✓ Saved company data to company.toml\n");
    
    // ============================================
    // EXAMPLE 7: YAML (Another format)
    // ============================================
    println!("--- EXAMPLE 7: YAML Format ---");
    
    let company_yaml = serde_yaml::to_string(&company).unwrap();
    println!("Company as YAML:\n{}\n", company_yaml);
    
    // Save to YAML file
    fs::write("company.yaml", &company_yaml)
        .expect("Failed to write YAML file");
    println!("✓ Saved company data to company.yaml\n");
    
    // ============================================
    // EXAMPLE 8: Convert between formats
    // ============================================
    println!("--- EXAMPLE 8: Format Conversion ---");
    
    // Start with JSON
    let original_json = r#"{"name": "Eve", "age": 29, "email": "eve@example.com", "is_active": true}"#;
    
    // Parse JSON → Person struct
    let person: Person = serde_json::from_str(original_json).unwrap();
    println!("Original JSON: {}", original_json);
    
    // Convert Person struct → YAML
    let as_yaml = serde_yaml::to_string(&person).unwrap();
    println!("Same data as YAML:\n{}", as_yaml);
    
    // Convert Person struct → TOML
    let as_toml = toml::to_string_pretty(&person).unwrap();
    println!("Same data as TOML:\n{}\n", as_toml);
    
    println!("=== ALL EXAMPLES COMPLETE ===");
    println!("\nCheck your project folder for:");
    println!("  - company.json");
    println!("  - company.toml");
    println!("  - company.yaml");
}

// Clone trait allows us to copy the struct
// This is needed for the examples above
impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),
            age: self.age,
            email: self.email.clone(),
            is_active: self.is_active,
        }
    }
}
