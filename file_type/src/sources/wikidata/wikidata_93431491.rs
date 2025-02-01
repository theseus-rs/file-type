use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_93431491: FileFormat = FileFormat {
    id: 93_431_491,
    puid: "wikidata/93431491",
    name: "Final Draft Document 8",
    extensions: &["fdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
