use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128624941: FileType = FileType {
    file_format: &FileFormat {
        id: 128_624_941,
        source_type: SourceType::Wikidata,
        name: "AutoIt file",
        extensions: &["au3"],
        media_types: &["text/x-autoit"],
        signatures: &[],
        related_formats: &[],
    },
};
