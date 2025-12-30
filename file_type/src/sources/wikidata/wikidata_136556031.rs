use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136556031: FileType = FileType {
    file_format: &FileFormat {
        id: 136_556_031,
        source_type: SourceType::Wikidata,
        name: "NX Card Image",
        extensions: &["xci"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
