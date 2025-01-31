use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857112: FileFormat = FileFormat {
    id: 105_857_112,
    puid: "wikidata/105857112",
    name: "XML Grammar",
    extensions: &["grxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
