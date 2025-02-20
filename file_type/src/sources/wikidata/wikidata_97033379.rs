use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_97033379: FileType = FileType {
    file_format: &FileFormat {
        id: 97_033_379,
        source_type: SourceType::Wikidata,
        name: "Magick Persistent Registry",
        extensions: &["mpr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
