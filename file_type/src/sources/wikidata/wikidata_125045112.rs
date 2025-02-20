use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125045112: FileType = FileType {
    file_format: &FileFormat {
        id: 125_045_112,
        source_type: SourceType::Wikidata,
        name: "Yoshimi Patch Set File",
        extensions: &["xmz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
