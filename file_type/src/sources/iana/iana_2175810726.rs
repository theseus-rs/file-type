use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2175810726: FileType = FileType {
    file_format: &FileFormat {
        id: 2_175_810_726,
        source_type: SourceType::Iana,
        name: "appledouble",
        extensions: &[],
        media_types: &["multipart/appledouble"],
        signatures: &[],
        related_formats: &[],
    },
};
