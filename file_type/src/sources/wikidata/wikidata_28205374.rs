use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205374: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_374,
        source_type: SourceType::Wikidata,
        name: "Leaf MOS",
        extensions: &["mos"],
        media_types: &["image/x-raw-leaf"],
        signatures: &[],
        related_formats: &[],
    },
};
