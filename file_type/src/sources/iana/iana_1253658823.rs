use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1253658823: FileType = FileType {
    file_format: &FileFormat {
        id: 1_253_658_823,
        source_type: SourceType::Iana,
        name: "vnd.evolv.ecig.theme",
        extensions: &[],
        media_types: &["application/vnd.evolv.ecig.theme"],
        signatures: &[],
        related_formats: &[],
    },
};
