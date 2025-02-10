use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110995856: FileType = FileType {
    file_format: &FileFormat {
        id: 110_995_856,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Card File format",
        extensions: &["car"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
