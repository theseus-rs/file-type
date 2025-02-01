use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862425: FileFormat = FileFormat {
    id: 105_862_425,
    puid: "wikidata/105862425",
    name: "DASH Media Presentation Description",
    extensions: &["mpd"],
    media_types: &["application/dash+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
