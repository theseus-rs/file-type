use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
