use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284101: FileFormat = FileFormat {
    id: 111_284_101,
    source_type: SourceType::Wikidata,
    name: "G.726 2-bit (16 kbps) ADPCM format data",
    extensions: &["g726-2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
