use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
