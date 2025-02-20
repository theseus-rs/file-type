use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130290522: FileType = FileType {
    file_format: &FileFormat {
        id: 130_290_522,
        source_type: SourceType::Wikidata,
        name: "Meson file format",
        extensions: &["meson.build"],
        media_types: &["text/x-meson"],
        signatures: &[],
        related_formats: &[],
    },
};
