use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110086768: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_768,
        source_type: SourceType::Wikidata,
        name: "Agisoft Project Archive",
        extensions: &["psz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
