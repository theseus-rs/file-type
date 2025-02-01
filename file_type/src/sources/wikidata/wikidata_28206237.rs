use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206237: FileFormat = FileFormat {
    id: 28_206_237,
    puid: "wikidata/28206237",
    name: "GROB",
    extensions: &["grb", "gro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
