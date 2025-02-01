use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111315905: FileFormat = FileFormat {
    id: 111_315_905,
    puid: "wikidata/111315905",
    name: "IBM MWave DSP synthesizer bank setup",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
