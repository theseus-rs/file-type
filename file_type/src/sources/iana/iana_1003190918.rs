use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1003190918: FileType = FileType {
    file_format: &FileFormat {
        id: 1_003_190_918,
        source_type: SourceType::Iana,
        name: "vnd.medcalcdata",
        extensions: &[],
        media_types: &["application/vnd.medcalcdata"],
        signatures: &[],
        related_formats: &[],
    },
};
