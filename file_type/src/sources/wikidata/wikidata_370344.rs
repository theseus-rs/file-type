use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_370344: FileType = FileType {
    file_format: &FileFormat {
        id: 370_344,
        source_type: SourceType::Wikidata,
        name: "Open Publication Distribution System Catalog",
        extensions: &[],
        media_types: &["application/atom+xml;profile=opds-catalog"],
        signatures: &[],
        related_formats: &[],
    },
};
