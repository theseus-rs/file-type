use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205535: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_535,
        source_type: SourceType::Wikidata,
        name: "iThmb",
        extensions: &["ithmb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
