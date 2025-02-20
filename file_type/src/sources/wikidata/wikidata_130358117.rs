use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130358117: FileType = FileType {
    file_format: &FileFormat {
        id: 130_358_117,
        source_type: SourceType::Wikidata,
        name: "Mosel source code file",
        extensions: &["mos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
