use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205982: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_982,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Q Color Channel",
        extensions: &["imq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
