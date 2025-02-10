use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117287703: FileType = FileType {
    file_format: &FileFormat {
        id: 117_287_703,
        source_type: SourceType::Wikidata,
        name: "SigmaStat DOS Worksheet",
        extensions: &["sp5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
