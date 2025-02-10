use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130619766: FileType = FileType {
    file_format: &FileFormat {
        id: 130_619_766,
        source_type: SourceType::Wikidata,
        name: "RelaxNG compact syntax file format",
        extensions: &["rnc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
