use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_945923: FileType = FileType {
    file_format: &FileFormat {
        id: 945_923,
        source_type: SourceType::Wikidata,
        name: "Web application ARchive",
        extensions: &["war"],
        media_types: &["application/java-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
