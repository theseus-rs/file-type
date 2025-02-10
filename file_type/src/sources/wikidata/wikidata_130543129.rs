use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130543129: FileType = FileType {
    file_format: &FileFormat {
        id: 130_543_129,
        source_type: SourceType::Wikidata,
        name: "Puppet configuration file format",
        extensions: &["pp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
