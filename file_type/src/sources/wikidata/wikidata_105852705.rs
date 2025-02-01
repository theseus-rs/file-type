use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852705: FileFormat = FileFormat {
    id: 105_852_705,
    puid: "wikidata/105852705",
    name: "GTKWave Saved session (deprecated)",
    extensions: &["sav"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x2A, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
