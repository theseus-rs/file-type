use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206830: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_830,
        source_type: SourceType::Wikidata,
        name: "Palette Master",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
