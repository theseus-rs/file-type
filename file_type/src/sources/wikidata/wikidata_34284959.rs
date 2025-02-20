use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34284959: FileType = FileType {
    file_format: &FileFormat {
        id: 34_284_959,
        source_type: SourceType::Wikidata,
        name: "Perl 6 script",
        extensions: &["pl6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
