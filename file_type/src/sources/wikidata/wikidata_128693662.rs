use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128693662: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_662,
        source_type: SourceType::Wikidata,
        name: "Befunge file format",
        extensions: &["befunge"],
        media_types: &["application/x-befunge"],
        signatures: &[],
        related_formats: &[],
    },
};
