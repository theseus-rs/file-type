use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856455: FileFormat = FileFormat {
    id: 105_856_455,
    source_type: SourceType::Wikidata,
    name: "Yokogawa waveform data",
    extensions: &["wvf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x57, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
