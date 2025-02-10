use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100597624: FileType = FileType {
    file_format: &FileFormat {
        id: 100_597_624,
        source_type: SourceType::Wikidata,
        name: "Minitab Project, version 15+",
        extensions: &["mpj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
