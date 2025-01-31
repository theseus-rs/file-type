use crate::format::file_format::FileFormat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub(crate) struct ReportFormatDetail {
    pub(crate) file_format: FileFormat,
}
