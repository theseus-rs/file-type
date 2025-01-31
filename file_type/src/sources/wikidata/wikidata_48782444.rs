use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48782444: FileFormat = FileFormat {
    id: 48_782_444,
    puid: "wikidata/48782444",
    name: "ACBM Graphics",
    extensions: &["acb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
