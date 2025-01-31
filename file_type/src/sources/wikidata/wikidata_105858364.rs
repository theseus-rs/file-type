use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858364: FileFormat = FileFormat {
    id: 105_858_364,
    puid: "wikidata/105858364",
    name: "ETABS model",
    extensions: &["edb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x54, 0x41, 0x42, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
