use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3459914175: FileType = FileType {
    file_format: &FileFormat {
        id: 3_459_914_175,
        source_type: SourceType::Iana,
        name: "vnd.sealedmedia.softseal.html",
        extensions: &[],
        media_types: &["application/vnd.sealedmedia.softseal.html"],
        signatures: &[],
        related_formats: &[],
    },
};
