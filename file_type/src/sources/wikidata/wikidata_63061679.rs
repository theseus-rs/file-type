use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63061679: FileType = FileType {
    file_format: &FileFormat {
        id: 63_061_679,
        source_type: SourceType::Wikidata,
        name: "Windows Bitmap, version 3.0 NT",
        extensions: &["bmp", "dib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
