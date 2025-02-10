use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110086818: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_818,
        source_type: SourceType::Wikidata,
        name: "Agisoft Project File",
        extensions: &["psx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
