use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27978795: FileType = FileType {
    file_format: &FileFormat {
        id: 27_978_795,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 Uncompressed",
        extensions: &["spu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
