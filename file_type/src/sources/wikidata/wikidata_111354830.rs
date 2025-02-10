use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111354830: FileType = FileType {
    file_format: &FileFormat {
        id: 111_354_830,
        source_type: SourceType::Wikidata,
        name: "Yamaha Tyros 2 custom drum voice file",
        extensions: &["tvd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
