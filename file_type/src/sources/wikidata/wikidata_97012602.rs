use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_97012602: FileType = FileType {
    file_format: &FileFormat {
        id: 97_012_602,
        source_type: SourceType::Wikidata,
        name: "gnuplot plot files",
        extensions: &["gp", "gplt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
