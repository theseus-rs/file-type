use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1571494: FileType = FileType {
    file_format: &FileFormat {
        id: 1_571_494,
        source_type: SourceType::Wikidata,
        name: "XProc",
        extensions: &["xpl"],
        media_types: &["application/xproc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
