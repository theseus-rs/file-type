use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47233611: FileFormat = FileFormat {
    id: 47_233_611,
    puid: "wikidata/47233611",
    name: "MPD",
    extensions: &["mpd"],
    media_types: &["application/x-multi-part-ldraw"],
    internal_signatures: &[],
    related_formats: &[],
};
