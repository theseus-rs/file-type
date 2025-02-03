use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863622: FileFormat = FileFormat {
    id: 105_863_622,
    source_type: SourceType::Wikidata,
    name: "LightWave Motion data",
    extensions: &["mot"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x57, 0x4D, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
