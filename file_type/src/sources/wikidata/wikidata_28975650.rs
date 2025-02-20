use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975650: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_650,
        source_type: SourceType::Wikidata,
        name: "Recon Mesh",
        extensions: &["m"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
