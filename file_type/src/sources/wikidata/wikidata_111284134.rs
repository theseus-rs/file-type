use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284134: FileFormat = FileFormat {
    id: 111_284_134,
    source_type: SourceType::Wikidata,
    name: "G.726 4-bit (32 kbps) ADPCM format data",
    extensions: &["g726-4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
