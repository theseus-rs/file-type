use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129825037: FileType = FileType {
    file_format: &FileFormat {
        id: 129_825_037,
        source_type: SourceType::Wikidata,
        name: "Io source code file",
        extensions: &["io"],
        media_types: &["text/x-iosrc"],
        signatures: &[],
        related_formats: &[],
    },
};
