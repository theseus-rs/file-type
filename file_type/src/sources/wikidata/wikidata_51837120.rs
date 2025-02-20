use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51837120: FileType = FileType {
    file_format: &FileFormat {
        id: 51_837_120,
        source_type: SourceType::Wikidata,
        name: "Scitex Continuous Tone Bitmap",
        extensions: &["ct"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
