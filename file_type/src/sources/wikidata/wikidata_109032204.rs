use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109032204: FileType = FileType {
    file_format: &FileFormat {
        id: 109_032_204,
        source_type: SourceType::Wikidata,
        name: "Zeiss Vision Image",
        extensions: &["zvi"],
        media_types: &["image/zvi"],
        signatures: &[],
        related_formats: &[],
    },
};
