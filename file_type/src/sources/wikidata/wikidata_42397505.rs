use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_42397505: FileFormat = FileFormat {
    id: 42_397_505,
    puid: "wikidata/42397505",
    name: "vimwiki",
    extensions: &["wiki"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
