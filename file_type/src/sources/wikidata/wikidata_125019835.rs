use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125019835: FileType = FileType {
    file_format: &FileFormat {
        id: 125_019_835,
        source_type: SourceType::Wikidata,
        name: "Sysex dump",
        extensions: &["syx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
