use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850782: FileFormat = FileFormat {
    id: 105_850_782,
    puid: "wikidata/105850782",
    name: "Keyman keyboard source (with rem)",
    extensions: &["kmn"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x63, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
