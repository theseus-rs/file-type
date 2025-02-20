use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205410: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_410,
        source_type: SourceType::Wikidata,
        name: "Nikon Dust File",
        extensions: &["ndf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
