use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27826417: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_417,
        source_type: SourceType::Wikidata,
        name: "ActionScript file format",
        extensions: &["as"],
        media_types: &[
            "application/ecmascript",
            "application/x-actionscript",
            "text/actionscript",
            "text/x-actionscript",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
