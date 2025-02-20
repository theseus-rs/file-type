use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132539785: FileType = FileType {
    file_format: &FileFormat {
        id: 132_539_785,
        source_type: SourceType::Wikidata,
        name: "Subsurface Material Properties file format",
        extensions: &["mprops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
