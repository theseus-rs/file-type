use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124844286: FileFormat = FileFormat {
    id: 124_844_286,
    puid: "wikidata/124844286",
    name: "CyberLink MediaShow Project",
    extensions: &["mbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
