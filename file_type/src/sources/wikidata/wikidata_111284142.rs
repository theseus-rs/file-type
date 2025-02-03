use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111284142: FileFormat = FileFormat {
    id: 111_284_142,
    source_type: SourceType::Wikidata,
    name: "G.726 5-bit (40 kbps) ADPCM format data",
    extensions: &["g726-5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
