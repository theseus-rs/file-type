use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118270989: FileType = FileType {
    file_format: &FileFormat {
        id: 118_270_989,
        source_type: SourceType::Wikidata,
        name: "Altamira Composer file",
        extensions: &["acc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
