use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661245: FileFormat = FileFormat {
    id: 112_661_245,
    puid: "wikidata/112661245",
    name: "Autodesk Inventor Assembly file format",
    extensions: &["iam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
