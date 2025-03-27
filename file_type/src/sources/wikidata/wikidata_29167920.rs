use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167920: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_920,
        source_type: SourceType::Wikidata,
        name: "Pack, Encrypt, Authenticate",
        extensions: &["pea"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
