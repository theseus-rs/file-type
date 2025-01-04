use file_type::format::FileFormat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ReportFormatDetail {
    file_format: FileFormat,
}

impl ReportFormatDetail {
    /// Returns the file format.
    pub fn file_format(&self) -> &FileFormat {
        &self.file_format
    }
}
