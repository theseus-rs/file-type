use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7395247: FileFormat = FileFormat {
    id: 7_395_247,
    puid: "wikidata/7395247",
    name: "SYBYL line notation",
    extensions: &["sln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
