use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134006393: FileType = FileType {
    file_format: &FileFormat {
        id: 134_006_393,
        source_type: SourceType::Wikidata,
        name: "Leo file",
        extensions: &["leo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
