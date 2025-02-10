use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205870: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_870,
        source_type: SourceType::Wikidata,
        name: "Crack Art",
        extensions: &["ca1", "ca2", "ca3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
