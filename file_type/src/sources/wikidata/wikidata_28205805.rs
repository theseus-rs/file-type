use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205805: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_805,
        source_type: SourceType::Wikidata,
        name: "Object File Format, binary variant",
        extensions: &["off"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
