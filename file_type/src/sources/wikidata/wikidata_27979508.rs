use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979508: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_508,
        source_type: SourceType::Wikidata,
        name: "RIFF Palette File",
        extensions: &["pal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
