use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5301: FileType = FileType {
    file_format: &FileFormat {
        id: 5_301,
        source_type: SourceType::Wikidata,
        name: "TeX",
        extensions: &["tex"],
        media_types: &["application/x-tex", "math/tex", "text/x-tex"],
        signatures: &[],
        related_formats: &[],
    },
};
