use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125150942: FileType = FileType {
    file_format: &FileFormat {
        id: 125_150_942,
        source_type: SourceType::Wikidata,
        name: "OmniGraffle Drawing (zipped)",
        extensions: &["graffle.zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
