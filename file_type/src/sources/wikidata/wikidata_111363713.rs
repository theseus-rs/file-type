use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363713: FileFormat = FileFormat {
    id: 111_363_713,
    puid: "wikidata/111363713",
    name: "Yamaha Motif XF 'waveforms' format",
    extensions: &["x3w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
