use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975868: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_868,
        source_type: SourceType::Wikidata,
        name: "OOGL SPHERE file",
        extensions: &["sph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
