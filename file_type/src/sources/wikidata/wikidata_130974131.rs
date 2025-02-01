use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130974131: FileFormat = FileFormat {
    id: 130_974_131,
    puid: "wikidata/130974131",
    name: "Silver source code file",
    extensions: &["sil", "ver"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
