use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110458180: FileType = FileType {
    file_format: &FileFormat {
        id: 110_458_180,
        source_type: SourceType::Wikidata,
        name: "Beam Software SIFF File",
        extensions: &["son", "vb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
