use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205948: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_948,
        source_type: SourceType::Wikidata,
        name: "Dr. Halo Bitmap",
        extensions: &["cut"],
        media_types: &["application/dr-halo", "image/x-cut"],
        signatures: &[],
        related_formats: &[],
    },
};
