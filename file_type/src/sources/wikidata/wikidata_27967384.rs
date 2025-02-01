use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967384: FileFormat = FileFormat {
    id: 27_967_384,
    puid: "wikidata/27967384",
    name: "SoundFont 2.0",
    extensions: &["sf2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
