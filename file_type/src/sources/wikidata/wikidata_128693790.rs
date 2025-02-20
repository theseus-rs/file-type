use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128693790: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_790,
        source_type: SourceType::Wikidata,
        name: "Boa file format",
        extensions: &["boa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
