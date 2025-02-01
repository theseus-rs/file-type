use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47539061: FileFormat = FileFormat {
    id: 47_539_061,
    puid: "wikidata/47539061",
    name: "AutoCAD dbConnect Template Set",
    extensions: &["dbt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
