use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966930: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_930,
        source_type: SourceType::Wikidata,
        name: "QSF",
        extensions: &["miniqsf", "qsf", "qsflib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
