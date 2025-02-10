use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_675481075: FileType = FileType {
    file_format: &FileFormat {
        id: 675_481_075,
        source_type: SourceType::Iana,
        name: "vnd.collection.doc+json",
        extensions: &[],
        media_types: &["application/vnd.collection.doc+json"],
        signatures: &[],
        related_formats: &[],
    },
};
