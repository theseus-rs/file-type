use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138644307: FileType = FileType {
    file_format: &FileFormat {
        id: 138_644_307,
        source_type: SourceType::Wikidata,
        name: "Transportable Layout Design",
        extensions: &["tld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
