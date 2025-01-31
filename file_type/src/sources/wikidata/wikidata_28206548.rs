use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206548: FileFormat = FileFormat {
    id: 28_206_548,
    puid: "wikidata/28206548",
    name: "MAKIchan Graphics MAX",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
