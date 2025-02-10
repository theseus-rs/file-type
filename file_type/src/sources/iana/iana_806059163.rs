use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_806059163: FileType = FileType {
    file_format: &FileFormat {
        id: 806_059_163,
        source_type: SourceType::Iana,
        name: "vnd.sealedmedia.softseal.gif",
        extensions: &[],
        media_types: &["image/vnd.sealedmedia.softseal.gif"],
        signatures: &[],
        related_formats: &[],
    },
};
