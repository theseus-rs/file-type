use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21848871: FileType = FileType {
    file_format: &FileFormat {
        id: 21_848_871,
        source_type: SourceType::Wikidata,
        name: "BYU",
        extensions: &["byu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
