use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867518: FileFormat = FileFormat {
    id: 105_867_518,
    puid: "wikidata/105867518",
    name: "Haines NFF scene",
    extensions: &["nff"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
