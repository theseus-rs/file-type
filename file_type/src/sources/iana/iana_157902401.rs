use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_157902401: FileType = FileType {
    file_format: &FileFormat {
        id: 157_902_401,
        source_type: SourceType::Iana,
        name: "vnd.fujixerox.edmics-mmr",
        extensions: &[],
        media_types: &["image/vnd.fujixerox.edmics-mmr"],
        signatures: &[],
        related_formats: &[],
    },
};
