use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110040777: FileFormat = FileFormat {
    id: 110_040_777,
    puid: "wikidata/110040777",
    name: "Harvard Graphics Presentation, version 4",
    extensions: &["pr4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
