use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3163454889: FileFormat = FileFormat {
    id: 3_163_454_889,
    source_type: SourceType::Iana,
    name: "alto-costmapfilter+json",
    extensions: &[],
    media_types: &["application/alto-costmapfilter+json"],
    signatures: &[],
    related_formats: &[],
};
