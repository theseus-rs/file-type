use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34285652: FileType = FileType {
    file_format: &FileFormat {
        id: 34_285_652,
        source_type: SourceType::Wikidata,
        name: "Perl Common Gateway Interface script",
        extensions: &["cgi", "fcgi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
