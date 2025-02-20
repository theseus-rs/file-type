use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126960671: FileType = FileType {
    file_format: &FileFormat {
        id: 126_960_671,
        source_type: SourceType::Wikidata,
        name: "Vala source file",
        extensions: &["vala", "vapi"],
        media_types: &["text/x-vala"],
        signatures: &[],
        related_formats: &[],
    },
};
