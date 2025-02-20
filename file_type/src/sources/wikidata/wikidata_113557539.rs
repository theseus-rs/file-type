use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113557539: FileType = FileType {
    file_format: &FileFormat {
        id: 113_557_539,
        source_type: SourceType::Wikidata,
        name: "Prassi CD Right Plus Image",
        extensions: &["gcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
