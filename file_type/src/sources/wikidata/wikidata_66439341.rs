use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66439341: FileType = FileType {
    file_format: &FileFormat {
        id: 66_439_341,
        source_type: SourceType::Wikidata,
        name: "Volkswriter file format",
        extensions: &["vw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
