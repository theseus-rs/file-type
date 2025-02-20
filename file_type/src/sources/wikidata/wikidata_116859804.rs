use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116859804: FileType = FileType {
    file_format: &FileFormat {
        id: 116_859_804,
        source_type: SourceType::Wikidata,
        name: "Peachtree Vendor List",
        extensions: &["csv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
