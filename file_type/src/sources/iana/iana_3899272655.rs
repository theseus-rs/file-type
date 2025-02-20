use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3899272655: FileType = FileType {
    file_format: &FileFormat {
        id: 3_899_272_655,
        source_type: SourceType::Iana,
        name: "speex",
        extensions: &[],
        media_types: &["audio/speex"],
        signatures: &[],
        related_formats: &[],
    },
};
