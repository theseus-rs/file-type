use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1624322153: FileType = FileType {
    file_format: &FileFormat {
        id: 1_624_322_153,
        source_type: SourceType::Iana,
        name: "vnd.ibm.modcap (OBSOLETED in favor of application/vnd.afpc.modca)",
        extensions: &[],
        media_types: &["application/vnd.ibm.modcap"],
        signatures: &[],
        related_formats: &[],
    },
};
