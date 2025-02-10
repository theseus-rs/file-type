use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128203989: FileType = FileType {
    file_format: &FileFormat {
        id: 128_203_989,
        source_type: SourceType::Wikidata,
        name: "TorqueScript file",
        extensions: &["cs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
