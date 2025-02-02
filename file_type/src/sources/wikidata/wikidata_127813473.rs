use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127813473: FileFormat = FileFormat {
    id: 127_813_473,
    source_type: SourceType::Wikidata,
    name: "OpenSCAD file format",
    extensions: &["scad"],
    media_types: &["application/x-openscad"],
    internal_signatures: &[],
    related_formats: &[],
};
