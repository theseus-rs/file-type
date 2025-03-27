use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26208225: FileType = FileType {
    file_format: &FileFormat {
        id: 26_208_225,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Wordprocessing Document, Strict, ISO/IEC 29500:2008, with Microsoft extensions",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
