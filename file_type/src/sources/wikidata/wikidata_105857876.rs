use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857876: FileFormat = FileFormat {
    id: 105_857_876,
    puid: "wikidata/105857876",
    name: "Infinity Engine user interface description (V1)",
    extensions: &["chu"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x55, 0x49, 0x56, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
