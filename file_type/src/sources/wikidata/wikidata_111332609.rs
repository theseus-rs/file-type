use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111332609: FileType = FileType {
    file_format: &FileFormat {
        id: 111_332_609,
        source_type: SourceType::Wikidata,
        name: "Orion Sampler program",
        extensions: &["osp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
