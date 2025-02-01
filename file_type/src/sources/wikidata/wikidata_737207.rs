use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_737207: FileFormat = FileFormat {
    id: 737_207,
    puid: "wikidata/737207",
    name: "RealVideo",
    extensions: &["rv"],
    media_types: &["application/vnd.rn-realmedia"],
    internal_signatures: &[],
    related_formats: &[],
};
