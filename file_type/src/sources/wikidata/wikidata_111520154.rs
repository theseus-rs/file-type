use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
