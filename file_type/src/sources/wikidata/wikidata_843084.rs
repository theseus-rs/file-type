use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_843084: FileType = FileType {
    file_format: &FileFormat {
        id: 843_084,
        source_type: SourceType::Wikidata,
        name: "Microsoft Document Imaging Format",
        extensions: &["mdi"],
        media_types: &["image/vnd.ms-modi"],
        signatures: &[],
        related_formats: &[],
    },
};
