use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_940967: FileType = FileType {
    file_format: &FileFormat {
        id: 940_967,
        source_type: SourceType::Wikidata,
        name: "Musepack",
        extensions: &["mp", "mp+", "mpc", "mpp"],
        media_types: &["audio/musepack", "audio/x-musepack"],
        signatures: &[],
        related_formats: &[],
    },
};
