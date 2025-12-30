use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136654648: FileType = FileType {
    file_format: &FileFormat {
        id: 136_654_648,
        source_type: SourceType::Wikidata,
        name: "Raku documentation file",
        extensions: &["rakudoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
