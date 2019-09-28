#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;

extern crate serde_yaml;

pub mod data_structure {



// serde defaults are kinda strange to deal with, these functions work but are uglyish to me
    fn default_tenure_start() -> String {
        "".to_string()
    }

    fn default_tenure_end() -> String {
        "now".to_string()
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Tenure {
        #[serde(default = "default_tenure_start")]
        pub start: String,
        #[serde(default = "default_tenure_end")]
        pub end: String
    }

    impl Default for Tenure {
        fn default() -> Self {
            Tenure {start: "now".to_string(), end: "never".to_string()}
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        DEV,
        SENIOR_DEV
    }

    impl Default for Role {
        fn default() -> Self {
            Role::DEV
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Project {
        pub name: String,
        #[serde(default)]
        pub period: Tenure,
        pub description: String,
        #[serde(default)]
        pub role: Role,
        pub stack: Vec<String>,
//    contributions: Vec<String>
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Job {
        pub tenure: Tenure,
        pub projects: Vec<Project>
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Person {
        pub name: String,
        pub jobs: Vec<Job>
    }
}


