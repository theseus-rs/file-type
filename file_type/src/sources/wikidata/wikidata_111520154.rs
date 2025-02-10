use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111520154: FileType = FileType {
    file_format: &FileFormat {
        id: 111_520_154,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcInfo DAT File (internal)",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
