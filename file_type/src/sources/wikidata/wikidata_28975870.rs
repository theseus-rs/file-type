use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975870: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_870,
        source_type: SourceType::Wikidata,
        name: "OOGL INST file",
        extensions: &["inst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
