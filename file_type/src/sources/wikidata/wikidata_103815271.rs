use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_103815271: FileType = FileType {
    file_format: &FileFormat {
        id: 103_815_271,
        source_type: SourceType::Wikidata,
        name: "WebP 2",
        extensions: &["wp2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
