use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117843750: FileType = FileType {
    file_format: &FileFormat {
        id: 117_843_750,
        source_type: SourceType::Wikidata,
        name: "IBM IOCA",
        extensions: &["ica"],
        media_types: &["image/x-ioca"],
        signatures: &[],
        related_formats: &[],
    },
};
