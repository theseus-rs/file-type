use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27578083: FileType = FileType {
    file_format: &FileFormat {
        id: 27_578_083,
        source_type: SourceType::Wikidata,
        name: "AppleDouble Resource Fork, version 2",
        extensions: &[],
        media_types: &["multipart/appledouble"],
        signatures: &[],
        related_formats: &[],
    },
};
