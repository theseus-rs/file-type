use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263338: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_338,
        source_type: SourceType::Wikidata,
        name: "DirectMusic Producer DLS file",
        extensions: &["dlp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
