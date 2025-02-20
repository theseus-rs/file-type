use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116785016: FileType = FileType {
    file_format: &FileFormat {
        id: 116_785_016,
        source_type: SourceType::Wikidata,
        name: "ISU file",
        extensions: &["isu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
