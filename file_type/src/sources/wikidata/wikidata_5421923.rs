use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5421923: FileType = FileType {
    file_format: &FileFormat {
        id: 5_421_923,
        source_type: SourceType::Wikidata,
        name: "Extensible MPEG-4 Textual Format",
        extensions: &["xmt"],
        media_types: &["application/mpeg4-iod-xmt"],
        signatures: &[],
        related_formats: &[],
    },
};
