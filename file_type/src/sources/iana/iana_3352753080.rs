use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3352753080: FileType = FileType {
    file_format: &FileFormat {
        id: 3_352_753_080,
        source_type: SourceType::Iana,
        name: "vnd.sealedmedia.softseal.mpeg",
        extensions: &[],
        media_types: &["audio/vnd.sealedmedia.softseal.mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
