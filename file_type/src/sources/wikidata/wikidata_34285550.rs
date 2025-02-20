use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34285550: FileType = FileType {
    file_format: &FileFormat {
        id: 34_285_550,
        source_type: SourceType::Wikidata,
        name: "Perl header",
        extensions: &["ph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
