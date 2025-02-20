use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_75539922: FileType = FileType {
    file_format: &FileFormat {
        id: 75_539_922,
        source_type: SourceType::Wikidata,
        name: "Ulead Private Data",
        extensions: &["upd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
