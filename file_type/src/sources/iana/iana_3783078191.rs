use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
