use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130726704: FileType = FileType {
    file_format: &FileFormat {
        id: 130_726_704,
        source_type: SourceType::Wikidata,
        name: "SARL source code file",
        extensions: &["sarl"],
        media_types: &["text/x-sarl"],
        signatures: &[],
        related_formats: &[],
    },
};
