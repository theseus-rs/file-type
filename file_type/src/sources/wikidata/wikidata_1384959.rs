use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1384959: FileType = FileType {
    file_format: &FileFormat {
        id: 1_384_959,
        source_type: SourceType::Wikidata,
        name: "Extensible Forms Description Language",
        extensions: &["frm", "xfd", "xfdl"],
        media_types: &["application/vnd.xfdl"],
        signatures: &[],
        related_formats: &[],
    },
};
