use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49619410: FileFormat = FileFormat {
    id: 49_619_410,
    puid: "wikidata/49619410",
    name: "Revit Family File",
    extensions: &["rfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
