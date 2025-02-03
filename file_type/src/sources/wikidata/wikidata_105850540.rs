use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850540: FileFormat = FileFormat {
    id: 105_850_540,
    source_type: SourceType::Wikidata,
    name: "Castle of the Winds saved Game",
    extensions: &["cwg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x77, 0x01, 0x01, 0x00, 0x43, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
