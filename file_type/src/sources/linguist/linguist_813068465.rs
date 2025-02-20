use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_813068465: FileType = FileType {
    file_format: &FileFormat {
        id: 813_068_465,
        source_type: SourceType::Linguist,
        name: "Noir",
        extensions: &["nr"],
        media_types: &["text/x-rustsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
