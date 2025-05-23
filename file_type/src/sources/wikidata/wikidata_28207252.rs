use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207252: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_252,
        source_type: SourceType::Wikidata,
        name: "SCR",
        extensions: &["scr"],
        media_types: &["image/x-zx-spectrum-standard-screen"],
        signatures: &[],
        related_formats: &[],
    },
};
