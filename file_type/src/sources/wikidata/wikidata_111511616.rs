use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111511616: FileType = FileType {
    file_format: &FileFormat {
        id: 111_511_616,
        source_type: SourceType::Wikidata,
        name: "SXG (ZX Spectrum) Graphic File",
        extensions: &["sxg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
