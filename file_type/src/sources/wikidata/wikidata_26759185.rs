use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26759185: FileFormat = FileFormat {
    id: 26_759_185,
    puid: "wikidata/26759185",
    name: "Drawing Interchange Binary Format",
    extensions: &["dxb"],
    media_types: &["application/x-dxb"],
    internal_signatures: &[],
    related_formats: &[],
};
