use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858277: FileFormat = FileFormat {
    id: 105_858_277,
    puid: "wikidata/105858277",
    name: "Entity Data Model",
    extensions: &["edmx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
