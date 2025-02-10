use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114131966: FileType = FileType {
    file_format: &FileFormat {
        id: 114_131_966,
        source_type: SourceType::Wikidata,
        name: "Chem3D template",
        extensions: &["c3t"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
