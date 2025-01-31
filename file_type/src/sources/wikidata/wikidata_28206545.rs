use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206545: FileFormat = FileFormat {
    id: 28_206_545,
    puid: "wikidata/28206545",
    name: "MAKIchan Graphics MAG",
    extensions: &["mag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
