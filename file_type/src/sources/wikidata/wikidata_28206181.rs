use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206181: FileFormat = FileFormat {
    id: 28_206_181,
    puid: "wikidata/28206181",
    name: "GIMP Parametric Brush",
    extensions: &["vbr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
