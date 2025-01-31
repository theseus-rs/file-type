use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131747923: FileFormat = FileFormat {
    id: 131_747_923,
    puid: "wikidata/131747923",
    name: "MNI polygonal surface mesh format",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
