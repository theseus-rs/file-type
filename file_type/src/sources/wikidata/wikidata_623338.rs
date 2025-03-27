use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_623338: FileType = FileType {
    file_format: &FileFormat {
        id: 623_338,
        source_type: SourceType::Wikidata,
        name: "Web Services Description Language",
        extensions: &["wsdl"],
        media_types: &["application/wsdl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
