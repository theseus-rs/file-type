use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_77142970: FileType = FileType {
    file_format: &FileFormat {
        id: 77_142_970,
        source_type: SourceType::Wikidata,
        name: "JSON Feed",
        extensions: &[],
        media_types: &["application/feed+json"],
        signatures: &[],
        related_formats: &[],
    },
};
