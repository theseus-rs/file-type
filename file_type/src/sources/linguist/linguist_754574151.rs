use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_754574151: FileType = FileType {
    file_format: &FileFormat {
        id: 754_574_151,
        source_type: SourceType::Linguist,
        name: "Dune",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
