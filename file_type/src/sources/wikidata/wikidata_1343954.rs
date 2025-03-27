use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1343954: FileType = FileType {
    file_format: &FileFormat {
        id: 1_343_954,
        source_type: SourceType::Wikidata,
        name: "Sitemaps",
        extensions: &["sitemap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
