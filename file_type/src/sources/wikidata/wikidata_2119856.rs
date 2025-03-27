use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2119856: FileType = FileType {
    file_format: &FileFormat {
        id: 2_119_856,
        source_type: SourceType::Wikidata,
        name: "Wireless Application Protocol Bitmap Format",
        extensions: &["wbmp"],
        media_types: &["image/vnd.wap.wbmp"],
        signatures: &[],
        related_formats: &[],
    },
};
