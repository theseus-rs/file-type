use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205401: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_401,
        source_type: SourceType::Wikidata,
        name: "Nikon Electronic Format (Coolscan variant)",
        extensions: &["nef"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
