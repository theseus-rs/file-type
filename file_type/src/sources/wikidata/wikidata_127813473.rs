use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127813473: FileFormat = FileFormat {
    id: 127_813_473,
    puid: "wikidata/127813473",
    name: "OpenSCAD file format",
    extensions: &["scad"],
    media_types: &["application/x-openscad"],
    internal_signatures: &[],
    related_formats: &[],
};
