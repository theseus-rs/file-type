use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49989460: FileType = FileType {
    file_format: &FileFormat {
        id: 49_989_460,
        source_type: SourceType::Wikidata,
        name: "ActiveX License Package file",
        extensions: &["lpk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
