use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130736862: FileType = FileType {
    file_format: &FileFormat {
        id: 130_736_862,
        source_type: SourceType::Wikidata,
        name: "Scalate Server Page file",
        extensions: &["ssp"],
        media_types: &["application/x-ssp"],
        signatures: &[],
        related_formats: &[],
    },
};
