use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_592700294: FileType = FileType {
    file_format: &FileFormat {
        id: 592_700_294,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-audit+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-audit+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
