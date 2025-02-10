use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_99973071: FileType = FileType {
    file_format: &FileFormat {
        id: 99_973_071,
        source_type: SourceType::Wikidata,
        name: "OmniPage Document 18",
        extensions: &["opd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
