use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119856975: FileFormat = FileFormat {
    id: 119_856_975,
    puid: "wikidata/119856975",
    name: "Streets & Trips File",
    extensions: &["est"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
