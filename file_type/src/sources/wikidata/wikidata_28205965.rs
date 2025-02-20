use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205965: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_965,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Green Channel",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
