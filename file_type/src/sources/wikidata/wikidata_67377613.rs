use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67377613: FileType = FileType {
    file_format: &FileFormat {
        id: 67_377_613,
        source_type: SourceType::Wikidata,
        name: "CaseWare 2005 Compressed file",
        extensions: &["ac_"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
