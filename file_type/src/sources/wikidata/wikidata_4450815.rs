use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4450815: FileType = FileType {
    file_format: &FileFormat {
        id: 4_450_815,
        source_type: SourceType::Wikidata,
        name: "DGCA",
        extensions: &["dgc"],
        media_types: &["application/x-dgc-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
