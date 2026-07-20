use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138090726: FileType = FileType {
    file_format: &FileFormat {
        id: 138_090_726,
        source_type: SourceType::Wikidata,
        name: "Signum! file format",
        extensions: &["sdo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
