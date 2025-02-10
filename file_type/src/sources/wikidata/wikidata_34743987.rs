use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34743987: FileType = FileType {
    file_format: &FileFormat {
        id: 34_743_987,
        source_type: SourceType::Wikidata,
        name: "Spark",
        extensions: &["spk"],
        media_types: &["archive"],
        signatures: &[],
        related_formats: &[],
    },
};
