use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979253: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_253,
        source_type: SourceType::Wikidata,
        name: "PCBoard",
        extensions: &["pcb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
