use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110238400: FileType = FileType {
    file_format: &FileFormat {
        id: 110_238_400,
        source_type: SourceType::Wikidata,
        name: "Screenwriter 6 file format",
        extensions: &["mmsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
