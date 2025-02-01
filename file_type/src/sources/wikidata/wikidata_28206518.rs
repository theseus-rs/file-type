use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206518: FileFormat = FileFormat {
    id: 28_206_518,
    puid: "wikidata/28206518",
    name: "Lucasfilm picture",
    extensions: &["lff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
