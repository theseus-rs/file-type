use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111315905: FileFormat = FileFormat {
    id: 111_315_905,
    source_type: SourceType::Wikidata,
    name: "IBM MWave DSP synthesizer bank setup",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
