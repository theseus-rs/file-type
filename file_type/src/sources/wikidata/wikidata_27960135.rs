use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960135: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_135,
        source_type: SourceType::Wikidata,
        name: "INRS-Telecom file",
        extensions: &["aud"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
