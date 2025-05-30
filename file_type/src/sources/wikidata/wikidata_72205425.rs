use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_72205425: FileType = FileType {
    file_format: &FileFormat {
        id: 72_205_425,
        source_type: SourceType::Wikidata,
        name: "Exchange Offline Address Book",
        extensions: &["lzx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
