use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111440679: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_679,
        source_type: SourceType::Wikidata,
        name: "Perl POD File",
        extensions: &["pod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
