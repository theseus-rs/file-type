use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_955017407: FileType = FileType {
    file_format: &FileFormat {
        id: 955_017_407,
        source_type: SourceType::Linguist,
        name: "Boogie",
        extensions: &["bpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
