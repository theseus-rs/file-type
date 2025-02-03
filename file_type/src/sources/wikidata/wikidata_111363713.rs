use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111363713: FileFormat = FileFormat {
    id: 111_363_713,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif XF 'waveforms' format",
    extensions: &["x3w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
