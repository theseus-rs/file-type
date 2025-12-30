use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2970013502: FileType = FileType {
    file_format: &FileFormat {
        id: 2_970_013_502,
        source_type: SourceType::Iana,
        name: "vnd.as207960.vas.config+jer",
        extensions: &[],
        media_types: &["application/vnd.as207960.vas.config+jer"],
        signatures: &[],
        related_formats: &[],
    },
};
