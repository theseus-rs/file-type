use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134236531: FileType = FileType {
    file_format: &FileFormat {
        id: 134_236_531,
        source_type: SourceType::Wikidata,
        name: "RQDA file format",
        extensions: &["rqda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
