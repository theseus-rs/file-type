use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2391033626: FileType = FileType {
    file_format: &FileFormat {
        id: 2_391_033_626,
        source_type: SourceType::Iana,
        name: "xml-patch+xml",
        extensions: &[],
        media_types: &["application/xml-patch+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
