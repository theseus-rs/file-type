use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127327949: FileFormat = FileFormat {
    id: 127_327_949,
    puid: "wikidata/127327949",
    name: "Coffeescript file",
    extensions: &["coffee"],
    media_types: &["text/coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};
