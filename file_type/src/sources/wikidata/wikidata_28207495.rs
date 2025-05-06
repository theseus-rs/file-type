use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207495: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_495,
        source_type: SourceType::Wikidata,
        name: "Wigmore Artist 64",
        extensions: &["a64", "wig"],
        media_types: &["image/x-artist-64"],
        signatures: &[],
        related_formats: &[],
    },
};
