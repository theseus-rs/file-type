use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136813679: FileType = FileType {
    file_format: &FileFormat {
        id: 136_813_679,
        source_type: SourceType::Wikidata,
        name: "Ahnenblatt genealogy file",
        extensions: &["ahn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
