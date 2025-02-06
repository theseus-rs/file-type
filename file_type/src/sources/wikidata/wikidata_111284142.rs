use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284142: FileFormat = FileFormat {
    id: 111_284_142,
    source_type: SourceType::Wikidata,
    name: "G.726 5-bit (40 kbps) ADPCM format data",
    extensions: &["g726-5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
