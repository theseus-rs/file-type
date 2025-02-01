use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131081636: FileFormat = FileFormat {
    id: 131_081_636,
    puid: "wikidata/131081636",
    name: "Snowball source code file",
    extensions: &["sbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
