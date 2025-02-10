use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117448679: FileType = FileType {
    file_format: &FileFormat {
        id: 117_448_679,
        source_type: SourceType::Wikidata,
        name: "Praat TextGrid",
        extensions: &["textgrid"],
        media_types: &["text/praat-textgrid"],
        signatures: &[],
        related_formats: &[],
    },
};
