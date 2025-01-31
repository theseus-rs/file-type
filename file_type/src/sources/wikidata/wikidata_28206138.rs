use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206138: FileFormat = FileFormat {
    id: 28_206_138,
    puid: "wikidata/28206138",
    name: "Freedom of Press Info",
    extensions: &["1", "fop"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
