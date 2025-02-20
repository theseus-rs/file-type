use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117287169: FileType = FileType {
    file_format: &FileFormat {
        id: 117_287_169,
        source_type: SourceType::Wikidata,
        name: "SigmaPlot Curve Fit file",
        extensions: &["fit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
