use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207379: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_379,
        source_type: SourceType::Wikidata,
        name: "TIFF for Fax eXtended",
        extensions: &["tfx", "tif", "tiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
