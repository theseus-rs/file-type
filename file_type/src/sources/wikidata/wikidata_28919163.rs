use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919163: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_163,
        source_type: SourceType::Wikidata,
        name: "Cult3D",
        extensions: &["cd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
