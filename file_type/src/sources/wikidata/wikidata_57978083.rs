use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_57978083: FileType = FileType {
    file_format: &FileFormat {
        id: 57_978_083,
        source_type: SourceType::Wikidata,
        name: "ASP Application Directive File",
        extensions: &["asax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
