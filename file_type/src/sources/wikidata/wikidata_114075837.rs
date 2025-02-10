use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114075837: FileType = FileType {
    file_format: &FileFormat {
        id: 114_075_837,
        source_type: SourceType::Wikidata,
        name: "Media Descriptor Sidecar File",
        extensions: &["mds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
