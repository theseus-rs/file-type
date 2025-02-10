use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_148294950: FileType = FileType {
    file_format: &FileFormat {
        id: 148_294_950,
        source_type: SourceType::Iana,
        name: "vnd.mdl-mbsdf",
        extensions: &[],
        media_types: &["application/vnd.mdl-mbsdf"],
        signatures: &[],
        related_formats: &[],
    },
};
