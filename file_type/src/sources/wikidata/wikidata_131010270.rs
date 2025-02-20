use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131010270: FileType = FileType {
    file_format: &FileFormat {
        id: 131_010_270,
        source_type: SourceType::Wikidata,
        name: "Smarty template file",
        extensions: &["tpl"],
        media_types: &["application/x-smarty"],
        signatures: &[],
        related_formats: &[],
    },
};
