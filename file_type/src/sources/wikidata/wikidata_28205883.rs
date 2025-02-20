use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205883: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_883,
        source_type: SourceType::Wikidata,
        name: "Desktop Color Separation",
        extensions: &["c", "dcs", "eps", "k", "m", "y"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
