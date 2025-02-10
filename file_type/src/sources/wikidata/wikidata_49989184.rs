use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49989184: FileType = FileType {
    file_format: &FileFormat {
        id: 49_989_184,
        source_type: SourceType::Wikidata,
        name: "Macro Enabled Microsoft Powerpoint",
        extensions: &["pptm"],
        media_types: &["application/vnd.openxmlformats"],
        signatures: &[],
        related_formats: &[],
    },
};
