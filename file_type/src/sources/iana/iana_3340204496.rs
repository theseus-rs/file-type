use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3340204496: FileType = FileType {
    file_format: &FileFormat {
        id: 3_340_204_496,
        source_type: SourceType::Iana,
        name: "slate",
        extensions: &[],
        media_types: &["application/slate"],
        signatures: &[],
        related_formats: &[],
    },
};
