use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206811: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_811,
        source_type: SourceType::Wikidata,
        name: "Paint.NET image",
        extensions: &["pdn"],
        media_types: &["image/x-paintnet"],
        signatures: &[],
        related_formats: &[],
    },
};
