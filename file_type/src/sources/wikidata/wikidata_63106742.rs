use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63106742: FileType = FileType {
    file_format: &FileFormat {
        id: 63_106_742,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Word Processor file 3-4 for Windows",
        extensions: &["wps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
