use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117287222: FileFormat = FileFormat {
    id: 117_287_222,
    puid: "wikidata/117287222",
    name: "SigmaPlot DOS Worksheet",
    extensions: &["spg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
