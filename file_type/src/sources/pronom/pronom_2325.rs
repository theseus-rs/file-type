use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2325: FileType = FileType {
    file_format: &FileFormat {
        id: 2_325,
        source_type: SourceType::Pronom,
        name: "Agisoft Project Archive",
        extensions: &["psz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
