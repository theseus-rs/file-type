use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_925235833: FileType = FileType {
    file_format: &FileFormat {
        id: 925_235_833,
        source_type: SourceType::Linguist,
        name: "EdgeQL",
        extensions: &["edgeql", "esdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
