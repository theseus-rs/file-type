use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25098907: FileType = FileType {
    file_format: &FileFormat {
        id: 25_098_907,
        source_type: SourceType::Wikidata,
        name: "PC Screen Font",
        extensions: &["psf", "psfu"],
        media_types: &["application/x-font-linux-psf"],
        signatures: &[],
        related_formats: &[],
    },
};
