use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26759185: FileType = FileType {
    file_format: &FileFormat {
        id: 26_759_185,
        source_type: SourceType::Wikidata,
        name: "Drawing Interchange Binary Format",
        extensions: &["dxb"],
        media_types: &["application/x-dxb"],
        signatures: &[],
        related_formats: &[],
    },
};
