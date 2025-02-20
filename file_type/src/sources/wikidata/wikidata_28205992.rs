use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205992: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_992,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Alpha Channel",
        extensions: &["ima"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
