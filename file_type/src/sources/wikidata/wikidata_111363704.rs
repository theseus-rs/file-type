use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363704: FileFormat = FileFormat {
    id: 111_363_704,
    puid: "wikidata/111363704",
    name: "Yamaha Motif XF 'all' format",
    extensions: &["x3a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
