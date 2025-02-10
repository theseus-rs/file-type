use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207080: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_080,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Names file",
        extensions: &["sdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
