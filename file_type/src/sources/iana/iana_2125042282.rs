use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2125042282: FileType = FileType {
    file_format: &FileFormat {
        id: 2_125_042_282,
        source_type: SourceType::Iana,
        name: "vnd.ibm.rights-management",
        extensions: &[],
        media_types: &["application/vnd.ibm.rights-management"],
        signatures: &[],
        related_formats: &[],
    },
};
