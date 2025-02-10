use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128693985: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_985,
        source_type: SourceType::Wikidata,
        name: "Brainfuck file",
        extensions: &["bf"],
        media_types: &["application/x-brainfuck"],
        signatures: &[],
        related_formats: &[],
    },
};
