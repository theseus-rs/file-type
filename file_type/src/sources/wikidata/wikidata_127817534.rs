use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127817534: FileFormat = FileFormat {
    id: 127_817_534,
    puid: "wikidata/127817534",
    name: "gp script",
    extensions: &["gp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
