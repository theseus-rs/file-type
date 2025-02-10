use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966966: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_966,
        source_type: SourceType::Wikidata,
        name: "Accolade MIDI File Format",
        extensions: &["mus"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
