use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121815076: FileType = FileType {
    file_format: &FileFormat {
        id: 121_815_076,
        source_type: SourceType::Wikidata,
        name: "Esko ArtPro File",
        extensions: &["ap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
