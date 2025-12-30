use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136654849: FileType = FileType {
    file_format: &FileFormat {
        id: 136_654_849,
        source_type: SourceType::Wikidata,
        name: "Raku test file",
        extensions: &["rakutest"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
