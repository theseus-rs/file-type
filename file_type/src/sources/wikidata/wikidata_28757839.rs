use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757839: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_839,
        source_type: SourceType::Wikidata,
        name: "Genecyst Backup RAM",
        extensions: &["gsv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
