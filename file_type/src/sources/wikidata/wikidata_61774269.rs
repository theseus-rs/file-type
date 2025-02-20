use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61774269: FileType = FileType {
    file_format: &FileFormat {
        id: 61_774_269,
        source_type: SourceType::Wikidata,
        name: "WavPack Binary, version 4",
        extensions: &["wv"],
        media_types: &["audio/x-wv"],
        signatures: &[],
        related_formats: &[],
    },
};
