use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51801521: FileType = FileType {
    file_format: &FileFormat {
        id: 51_801_521,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Workspace",
        extensions: &["xlw"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[],
    },
};
