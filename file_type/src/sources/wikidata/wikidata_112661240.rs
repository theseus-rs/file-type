use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661240: FileFormat = FileFormat {
    id: 112_661_240,
    puid: "wikidata/112661240",
    name: "Autodesk Inventor Part file format",
    extensions: &["ipt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
