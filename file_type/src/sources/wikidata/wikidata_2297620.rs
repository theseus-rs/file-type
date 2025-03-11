use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2297620: FileType = FileType {
    file_format: &FileFormat {
        id: 2_297_620,
        source_type: SourceType::Wikidata,
        name: "Software Package Data Exchange",
        extensions: &[],
        media_types: &["text/spdx"],
        signatures: &[],
        related_formats: &[],
    },
};
