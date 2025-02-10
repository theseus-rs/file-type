use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_507: FileType = FileType {
    file_format: &FileFormat {
        id: 507,
        source_type: SourceType::Pronom,
        name: "Microsoft Visual FoxPro Table",
        extensions: &["dbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
