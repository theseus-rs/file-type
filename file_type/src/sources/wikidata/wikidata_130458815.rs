use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130458815: FileType = FileType {
    file_format: &FileFormat {
        id: 130_458_815,
        source_type: SourceType::Wikidata,
        name: "ParaSail source code",
        extensions: &["psi"],
        media_types: &["text/x-parasail"],
        signatures: &[],
        related_formats: &[],
    },
};
