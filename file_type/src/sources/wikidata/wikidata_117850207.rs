use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117850207: FileType = FileType {
    file_format: &FileFormat {
        id: 117_850_207,
        source_type: SourceType::Wikidata,
        name: "Xerox MicroFax",
        extensions: &["mif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
