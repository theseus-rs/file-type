use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1994: FileType = FileType {
    file_format: &FileFormat {
        id: 1_994,
        source_type: SourceType::Pronom,
        name: "InDesign Markup Language Package",
        extensions: &["idml"],
        media_types: &["application/vnd.adobe.indesign-idml-package"],
        signatures: &[],
        related_formats: &[],
    },
};
