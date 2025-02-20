use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3399265095: FileType = FileType {
    file_format: &FileFormat {
        id: 3_399_265_095,
        source_type: SourceType::Iana,
        name: "xml-external-parsed-entity",
        extensions: &[],
        media_types: &["text/xml-external-parsed-entity"],
        signatures: &[],
        related_formats: &[],
    },
};
