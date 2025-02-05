use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111315905: FileFormat = FileFormat {
    id: 111_315_905,
    source_type: SourceType::Wikidata,
    name: "IBM MWave DSP synthesizer bank setup",
    extensions: &["ini"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
