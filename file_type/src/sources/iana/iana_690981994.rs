use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_690981994: FileType = FileType {
    file_format: &FileFormat {
        id: 690_981_994,
        source_type: SourceType::Iana,
        name: "vnd.artsquare",
        extensions: &[],
        media_types: &["application/vnd.artsquare"],
        signatures: &[],
        related_formats: &[],
    },
};
