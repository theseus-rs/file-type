use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51839234: FileType = FileType {
    file_format: &FileFormat {
        id: 51_839_234,
        source_type: SourceType::Wikidata,
        name: "Inset Systems Bitmap",
        extensions: &["pix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
