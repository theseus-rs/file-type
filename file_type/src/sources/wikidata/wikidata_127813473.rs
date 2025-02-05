use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127813473: FileFormat = FileFormat {
    id: 127_813_473,
    source_type: SourceType::Wikidata,
    name: "OpenSCAD file format",
    extensions: &["scad"],
    media_types: &["application/x-openscad"],
    signatures: &[],
    related_formats: &[],
};
