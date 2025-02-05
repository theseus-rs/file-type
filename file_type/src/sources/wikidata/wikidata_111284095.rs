use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284095: FileFormat = FileFormat {
    id: 111_284_095,
    source_type: SourceType::Wikidata,
    name: "G.723 5-bit (40 kbps) ADPCM format data",
    extensions: &["g723-5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
