use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2238: FileType = FileType {
    file_format: &FileFormat {
        id: 2_238,
        source_type: SourceType::Pronom,
        name: "Corel Print House/Print Office Document",
        extensions: &["cph", "cpd", "cpo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
