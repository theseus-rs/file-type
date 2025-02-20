use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120867970: FileType = FileType {
    file_format: &FileFormat {
        id: 120_867_970,
        source_type: SourceType::Wikidata,
        name: "Cumulus Query Exchange File",
        extensions: &["cqe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
