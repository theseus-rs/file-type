use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207270: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_270,
        source_type: SourceType::Wikidata,
        name: "Secret Photos puzzle",
        extensions: &["xp0"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
