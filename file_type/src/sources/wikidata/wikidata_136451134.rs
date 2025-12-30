use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136451134: FileType = FileType {
    file_format: &FileFormat {
        id: 136_451_134,
        source_type: SourceType::Wikidata,
        name: "DaVinci Resolve Timeline File",
        extensions: &["drt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
