use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67124083: FileType = FileType {
    file_format: &FileFormat {
        id: 67_124_083,
        source_type: SourceType::Wikidata,
        name: "Print Artist label file format",
        extensions: &["lbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
