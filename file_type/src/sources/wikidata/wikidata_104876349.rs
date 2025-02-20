use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_104876349: FileType = FileType {
    file_format: &FileFormat {
        id: 104_876_349,
        source_type: SourceType::Wikidata,
        name: "JCAMP-DX",
        extensions: &["dx", "jcm", "jdx"],
        media_types: &["chemical/x-jcamp-dx"],
        signatures: &[],
        related_formats: &[],
    },
};
