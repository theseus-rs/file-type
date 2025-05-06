use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134286353: FileType = FileType {
    file_format: &FileFormat {
        id: 134_286_353,
        source_type: SourceType::Wikidata,
        name: "Clipper memo field file",
        extensions: &["dbt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
