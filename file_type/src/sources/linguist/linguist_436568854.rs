use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_436568854: FileType = FileType {
    file_format: &FileFormat {
        id: 436_568_854,
        source_type: SourceType::Linguist,
        name: "Protocol Buffer Text Format",
        extensions: &["pbt", "pbtxt", "textproto"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
