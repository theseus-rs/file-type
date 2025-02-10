use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2344313610: FileType = FileType {
    file_format: &FileFormat {
        id: 2_344_313_610,
        source_type: SourceType::Iana,
        name: "vnd.ciedi",
        extensions: &[],
        media_types: &["application/vnd.ciedi"],
        signatures: &[],
        related_formats: &[],
    },
};
