use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2508876996: FileType = FileType {
    file_format: &FileFormat {
        id: 2_508_876_996,
        source_type: SourceType::Iana,
        name: "vnd.pytha.pyox",
        extensions: &[],
        media_types: &["model/vnd.pytha.pyox"],
        signatures: &[],
        related_formats: &[],
    },
};
