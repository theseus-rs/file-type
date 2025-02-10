use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1269969670: FileType = FileType {
    file_format: &FileFormat {
        id: 1_269_969_670,
        source_type: SourceType::Iana,
        name: "vnd.RenLearn.rlprint",
        extensions: &[],
        media_types: &["application/vnd.RenLearn.rlprint"],
        signatures: &[],
        related_formats: &[],
    },
};
