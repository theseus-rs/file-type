use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111284556: FileType = FileType {
    file_format: &FileFormat {
        id: 111_284_556,
        source_type: SourceType::Wikidata,
        name: "GigaStudio/GigaSampler file",
        extensions: &["gi!", "gig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
