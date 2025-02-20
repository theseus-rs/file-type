use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_891857693: FileType = FileType {
    file_format: &FileFormat {
        id: 891_857_693,
        source_type: SourceType::Iana,
        name: "vnd.minisoft-hp3000-save",
        extensions: &[],
        media_types: &["application/vnd.minisoft-hp3000-save"],
        signatures: &[],
        related_formats: &[],
    },
};
