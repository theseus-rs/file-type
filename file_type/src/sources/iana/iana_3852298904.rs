use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3852298904: FileType = FileType {
    file_format: &FileFormat {
        id: 3_852_298_904,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-audit-conf+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-audit-conf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
