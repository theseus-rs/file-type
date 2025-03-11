use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_31792645: FileType = FileType {
    file_format: &FileFormat {
        id: 31_792_645,
        source_type: SourceType::Wikidata,
        name: "Web Annotation Data Model",
        extensions: &[],
        media_types: &["application/ld+json;profile=\"http://www.w3.org/ns/anno.jsonld\""],
        signatures: &[],
        related_formats: &[],
    },
};
