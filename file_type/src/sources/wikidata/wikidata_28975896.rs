use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975896: FileFormat = FileFormat {
    id: 28_975_896,
    puid: "wikidata/28975896",
    name: "Spline Control File",
    extensions: &["spl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
