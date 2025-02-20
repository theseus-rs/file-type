use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_56827097: FileType = FileType {
    file_format: &FileFormat {
        id: 56_827_097,
        source_type: SourceType::Wikidata,
        name: "Path of Exile GGPK",
        extensions: &["ggpk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
