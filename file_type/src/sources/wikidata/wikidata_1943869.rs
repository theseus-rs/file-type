use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1943869: FileType = FileType {
    file_format: &FileFormat {
        id: 1_943_869,
        source_type: SourceType::Wikidata,
        name: "oEmbed",
        extensions: &[],
        media_types: &["application/json+oembed", "text/xml+oembed"],
        signatures: &[],
        related_formats: &[],
    },
};
