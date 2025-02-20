use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116648074: FileType = FileType {
    file_format: &FileFormat {
        id: 116_648_074,
        source_type: SourceType::Wikidata,
        name: "TopLevel Forms Document",
        extensions: &["tfm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
