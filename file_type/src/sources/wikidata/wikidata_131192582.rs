use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131192582: FileType = FileType {
    file_format: &FileFormat {
        id: 131_192_582,
        source_type: SourceType::Wikidata,
        name: "Uxntal source code file",
        extensions: &["tal"],
        media_types: &["text/x-uxntal"],
        signatures: &[],
        related_formats: &[],
    },
};
