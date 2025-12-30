use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136678574: FileType = FileType {
    file_format: &FileFormat {
        id: 136_678_574,
        source_type: SourceType::Wikidata,
        name: "Compucon EOS Design File",
        extensions: &["che"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
