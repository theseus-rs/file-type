use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3783078191: FileType = FileType {
    file_format: &FileFormat {
        id: 3_783_078_191,
        source_type: SourceType::Iana,
        name: "vnd.belightsoft.lhzd+zip",
        extensions: &[],
        media_types: &["application/vnd.belightsoft.lhzd+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
