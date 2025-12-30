use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136700618: FileType = FileType {
    file_format: &FileFormat {
        id: 136_700_618,
        source_type: SourceType::Wikidata,
        name: "Super Nintendo Entertainment System ROM",
        extensions: &["sfc"],
        media_types: &["application/x-snes-rom"],
        signatures: &[],
        related_formats: &[],
    },
};
