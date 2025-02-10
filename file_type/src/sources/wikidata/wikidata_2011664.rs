use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2011664: FileType = FileType {
    file_format: &FileFormat {
        id: 2_011_664,
        source_type: SourceType::Wikidata,
        name: "Object File Format",
        extensions: &["off"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
