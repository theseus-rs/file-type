use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_183454639: FileType = FileType {
    file_format: &FileFormat {
        id: 183_454_639,
        source_type: SourceType::Iana,
        name: "vnd.ipld.car",
        extensions: &[],
        media_types: &["application/vnd.ipld.car"],
        signatures: &[],
        related_formats: &[],
    },
};
