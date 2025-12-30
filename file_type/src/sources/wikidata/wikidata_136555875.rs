use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136555875: FileType = FileType {
    file_format: &FileFormat {
        id: 136_555_875,
        source_type: SourceType::Wikidata,
        name: "iNES",
        extensions: &["nes"],
        media_types: &["application/x-nes-rom"],
        signatures: &[],
        related_formats: &[],
    },
};
