use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975766: FileFormat = FileFormat {
    id: 28_975_766,
    puid: "wikidata/28975766",
    name: "DMO format",
    extensions: &["dmo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
