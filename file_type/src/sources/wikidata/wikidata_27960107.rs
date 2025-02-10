use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27960107: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_107,
        source_type: SourceType::Wikidata,
        name: "Berkeley/IRCAM/Carl Sound Format",
        extensions: &["sf"],
        media_types: &["audio/x-ircam"],
        signatures: &[],
        related_formats: &[],
    },
};
