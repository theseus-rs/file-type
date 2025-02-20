use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117804843: FileType = FileType {
    file_format: &FileFormat {
        id: 117_804_843,
        source_type: SourceType::Wikidata,
        name: "Working Model 2D File",
        extensions: &["wm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
