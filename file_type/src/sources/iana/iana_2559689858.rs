use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2559689858: FileType = FileType {
    file_format: &FileFormat {
        id: 2_559_689_858,
        source_type: SourceType::Iana,
        name: "vnd.fujitsu.oasysgp",
        extensions: &[],
        media_types: &["application/vnd.fujitsu.oasysgp"],
        signatures: &[],
        related_formats: &[],
    },
};
