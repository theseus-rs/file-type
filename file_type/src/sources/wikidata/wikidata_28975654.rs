use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975654: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_654,
        source_type: SourceType::Wikidata,
        name: "Recon Points",
        extensions: &["pts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
