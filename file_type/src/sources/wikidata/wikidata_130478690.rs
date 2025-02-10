use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130478690: FileType = FileType {
    file_format: &FileFormat {
        id: 130_478_690,
        source_type: SourceType::Wikidata,
        name: "Pike source code file",
        extensions: &["pike"],
        media_types: &["text/x-pike"],
        signatures: &[],
        related_formats: &[],
    },
};
