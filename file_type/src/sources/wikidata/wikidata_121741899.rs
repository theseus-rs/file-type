use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121741899: FileType = FileType {
    file_format: &FileFormat {
        id: 121_741_899,
        source_type: SourceType::Wikidata,
        name: "TurboTax 2008 Tax Return",
        extensions: &["tax2008"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
