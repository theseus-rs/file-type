use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118165539: FileFormat = FileFormat {
    id: 118_165_539,
    puid: "wikidata/118165539",
    name: "FotoFinish Image Format",
    extensions: &["sph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
