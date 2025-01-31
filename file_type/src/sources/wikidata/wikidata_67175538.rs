use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67175538: FileFormat = FileFormat {
    id: 67_175_538,
    puid: "wikidata/67175538",
    name: "Nero Digital file",
    extensions: &["nd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
