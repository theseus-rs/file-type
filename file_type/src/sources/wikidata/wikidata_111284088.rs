use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284088: FileFormat = FileFormat {
    id: 111_284_088,
    source_type: SourceType::Wikidata,
    name: "G.723 3/5-bit ADPCM format data",
    extensions: &["g723"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
