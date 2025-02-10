use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2478971919: FileType = FileType {
    file_format: &FileFormat {
        id: 2_478_971_919,
        source_type: SourceType::Iana,
        name: "vnd.sealedmedia.softseal.mov",
        extensions: &[],
        media_types: &["video/vnd.sealedmedia.softseal.mov"],
        signatures: &[],
        related_formats: &[],
    },
};
