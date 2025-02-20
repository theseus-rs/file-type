use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21848765: FileType = FileType {
    file_format: &FileFormat {
        id: 21_848_765,
        source_type: SourceType::Wikidata,
        name: "BioSemi Data Format",
        extensions: &["bdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
