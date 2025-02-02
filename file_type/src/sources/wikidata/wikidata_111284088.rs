use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111284088: FileFormat = FileFormat {
    id: 111_284_088,
    source_type: SourceType::Wikidata,
    name: "G.723 3/5-bit ADPCM format data",
    extensions: &["g723"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
