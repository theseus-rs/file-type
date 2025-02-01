use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125916061: FileFormat = FileFormat {
    id: 125_916_061,
    puid: "wikidata/125916061",
    name: "SolidWorks Material Database File",
    extensions: &["sldmat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
