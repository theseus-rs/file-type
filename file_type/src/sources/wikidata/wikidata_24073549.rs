use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_24073549: FileFormat = FileFormat {
    id: 24_073_549,
    puid: "wikidata/24073549",
    name: "SFZ",
    extensions: &["sfz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
