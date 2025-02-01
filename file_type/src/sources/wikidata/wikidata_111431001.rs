use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111431001: FileFormat = FileFormat {
    id: 111_431_001,
    puid: "wikidata/111431001",
    name: "ExtendScript Included Script File",
    extensions: &["jsxinc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
