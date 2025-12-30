use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136555932: FileType = FileType {
    file_format: &FileFormat {
        id: 136_555_932,
        source_type: SourceType::Wikidata,
        name: "NES 2.0",
        extensions: &["nes"],
        media_types: &["application/x-nes-rom"],
        signatures: &[],
        related_formats: &[],
    },
};
