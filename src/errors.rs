use std::io::{ Error as IOError };
use std::fmt::{ Display, Formatter, Result as FmtResult };

use glob::PatternError;
use grok::Error as GrokError;
use serde_json::Error as SerdeError;
use serde_yaml::Error as SerdeYamlError;
use serde::{ Serialize, Deserialize };




#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    EnvVarError( String ),
    SimpleMsgError( String ),
    SerdeError( String ),
    IOError( String ),
    ConfigError( String ),
    PatternError( String ),
    GrokError( String ),
    IndexError
}

impl From<SerdeError> for ApiError {
    fn from( err: SerdeError ) -> ApiError {
        ApiError::SerdeError( err.to_string() )
    }
}
impl From<SerdeYamlError> for ApiError {
    fn from( err: SerdeYamlError ) -> ApiError {
        ApiError::SerdeError( err.to_string() )
    }
}
impl From<IOError> for ApiError {
    fn from( err: IOError ) -> ApiError {
        ApiError::IOError( err.to_string() )
    }
}
impl From<PatternError> for ApiError {
    fn from( err: PatternError ) -> ApiError {
        ApiError::PatternError( err.to_string() )
    }
}
impl From<GrokError> for ApiError {
    fn from( err: GrokError ) -> ApiError {
        ApiError::GrokError( err.to_string() )
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self {
            Self::EnvVarError( t ) => format!("EnvVarError: {}", t.to_string()),
            Self::SimpleMsgError( t ) => format!("SimpleMsgError: {}", t.to_string()),
            Self::IOError( t ) => format!("IOError: {}", t.to_string()),
            Self::ConfigError( t ) => format!("ConfigError: {}", t.to_string()),
            Self::SerdeError( t ) => format!("SessionDependencyError: {}", t.to_string()),
            Self::PatternError( t ) => format!("PatternError: {}", t.to_string()),
            Self::GrokError( t ) => format!("GrokError: {}", t.to_string()),
            Self::IndexError => format!( "IndexGettingError" )
        };
        write!(f, "{}", msg)
    }
}
