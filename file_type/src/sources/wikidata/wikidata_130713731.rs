use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130713731: FileFormat = FileFormat {
    id: 130_713_731,
    puid: "wikidata/130713731",
    name: "RSL file format",
    extensions: &["rsl"],
    media_types: &["text/rsl"],
    internal_signatures: &[],
    related_formats: &[],
};
