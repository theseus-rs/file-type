use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1108063713: FileType = FileType {
    file_format: &FileFormat {
        id: 1_108_063_713,
        source_type: SourceType::Iana,
        name: "prs.rdf-xml-crypt",
        extensions: &[],
        media_types: &["application/prs.rdf-xml-crypt"],
        signatures: &[],
        related_formats: &[],
    },
};
