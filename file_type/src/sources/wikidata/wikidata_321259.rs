use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_321259: FileType = FileType {
    file_format: &FileFormat {
        id: 321_259,
        source_type: SourceType::Wikidata,
        name: "Q321259",
        extensions: &["vcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
