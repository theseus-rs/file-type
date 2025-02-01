use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52426198: FileFormat = FileFormat {
    id: 52_426_198,
    puid: "wikidata/52426198",
    name: "XYWrite for Windows Document, version 4",
    extensions: &["xy4", "xy4", "xyw", "xyw"],
    media_types: &[
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
