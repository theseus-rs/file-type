use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4227995: FileType = FileType {
    file_format: &FileFormat {
        id: 4_227_995,
        source_type: SourceType::Wikidata,
        name: "eMule collection",
        extensions: &["emulecollection"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
