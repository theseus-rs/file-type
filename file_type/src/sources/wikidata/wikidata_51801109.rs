use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51801109: FileType = FileType {
    file_format: &FileFormat {
        id: 51_801_109,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Toolbar",
        extensions: &["xlb"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[],
    },
};
