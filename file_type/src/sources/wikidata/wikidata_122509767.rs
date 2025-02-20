use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122509767: FileType = FileType {
    file_format: &FileFormat {
        id: 122_509_767,
        source_type: SourceType::Wikidata,
        name: "Pretty Good Privacy (PGP) Groups Data",
        extensions: &["pgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
