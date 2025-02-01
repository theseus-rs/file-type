use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857407: FileFormat = FileFormat {
    id: 105_857_407,
    puid: "wikidata/105857407",
    name: "jalbum image info",
    extensions: &["jpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
