use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111262994: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_994,
        source_type: SourceType::Wikidata,
        name: "Aureal 'Aspen' bank file",
        extensions: &["arl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
