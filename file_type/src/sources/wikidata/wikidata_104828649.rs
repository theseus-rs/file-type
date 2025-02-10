use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_104828649: FileType = FileType {
    file_format: &FileFormat {
        id: 104_828_649,
        source_type: SourceType::Wikidata,
        name: "Renoise DSP device chain",
        extensions: &["xrnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
