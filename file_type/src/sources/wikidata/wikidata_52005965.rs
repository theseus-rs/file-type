use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52005965: FileType = FileType {
    file_format: &FileFormat {
        id: 52_005_965,
        source_type: SourceType::Wikidata,
        name: "Micrografx Draw, version 3",
        extensions: &["drw"],
        media_types: &["application/x-mgx-designer"],
        signatures: &[],
        related_formats: &[],
    },
};
