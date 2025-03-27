use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757747: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_747,
        source_type: SourceType::Wikidata,
        name: "GEM bitmap font",
        extensions: &["fnt", "gft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
