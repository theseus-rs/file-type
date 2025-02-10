use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959817: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_817,
        source_type: SourceType::Wikidata,
        name: "Ableton Max for Live Device",
        extensions: &["amxd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
