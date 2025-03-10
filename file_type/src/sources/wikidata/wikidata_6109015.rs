use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6109015: FileType = FileType {
    file_format: &FileFormat {
        id: 6_109_015,
        source_type: SourceType::Wikidata,
        name: "JScript.Encode",
        extensions: &[],
        media_types: &["text/jscript.encode"],
        signatures: &[],
        related_formats: &[],
    },
};
