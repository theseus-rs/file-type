use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854221: FileFormat = FileFormat {
    id: 105_854_221,
    puid: "wikidata/105854221",
    name: "PS/2 MicroChannel Adapter Description File (with CRLF)",
    extensions: &["adf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
