use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110501470: FileFormat = FileFormat {
    id: 110_501_470,
    puid: "wikidata/110501470",
    name: "reStructuredText format",
    extensions: &["rst"],
    media_types: &["text/x-rst"],
    internal_signatures: &[],
    related_formats: &[],
};
