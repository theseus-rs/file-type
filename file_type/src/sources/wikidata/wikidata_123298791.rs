use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298791: FileFormat = FileFormat {
    id: 123_298_791,
    puid: "wikidata/123298791",
    name: "Retrospect RXT File",
    extensions: &["rxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
