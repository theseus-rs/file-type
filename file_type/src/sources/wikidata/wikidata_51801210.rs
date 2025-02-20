use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51801210: FileType = FileType {
    file_format: &FileFormat {
        id: 51_801_210,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Chart, version 29",
        extensions: &["xlc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
