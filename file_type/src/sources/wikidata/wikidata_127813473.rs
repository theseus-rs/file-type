use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127813473: FileType = FileType {
    file_format: &FileFormat {
        id: 127_813_473,
        source_type: SourceType::Wikidata,
        name: "OpenSCAD file format",
        extensions: &["scad"],
        media_types: &["application/x-openscad"],
        signatures: &[],
        related_formats: &[],
    },
};
