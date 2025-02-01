use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967220: FileFormat = FileFormat {
    id: 27_967_220,
    puid: "wikidata/27967220",
    name: "SoundFX module",
    extensions: &["sfx", "sfx2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
