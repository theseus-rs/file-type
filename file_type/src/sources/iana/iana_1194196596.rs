use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1194196596: FileType = FileType {
    file_format: &FileFormat {
        id: 1_194_196_596,
        source_type: SourceType::Iana,
        name: "ulpfec",
        extensions: &[],
        media_types: &["audio/ulpfec"],
        signatures: &[],
        related_formats: &[],
    },
};
