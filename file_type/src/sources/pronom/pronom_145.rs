use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_145: FileType = FileType {
    file_format: &FileFormat {
        id: 145,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel OLE DB Query",
        extensions: &["rqy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
