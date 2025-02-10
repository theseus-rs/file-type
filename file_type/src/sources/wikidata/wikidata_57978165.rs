use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_57978165: FileType = FileType {
    file_format: &FileFormat {
        id: 57_978_165,
        source_type: SourceType::Wikidata,
        name: "ASP WebService Directive File",
        extensions: &["asmx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
