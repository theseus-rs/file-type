use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111284134: FileFormat = FileFormat {
    id: 111_284_134,
    source_type: SourceType::Wikidata,
    name: "G.726 4-bit (32 kbps) ADPCM format data",
    extensions: &["g726-4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
