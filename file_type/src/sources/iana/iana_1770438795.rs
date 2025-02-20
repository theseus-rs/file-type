use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1770438795: FileType = FileType {
    file_format: &FileFormat {
        id: 1_770_438_795,
        source_type: SourceType::Iana,
        name: "vnd.fujifilm.fb.docuworks",
        extensions: &[],
        media_types: &["application/vnd.fujifilm.fb.docuworks"],
        signatures: &[],
        related_formats: &[],
    },
};
