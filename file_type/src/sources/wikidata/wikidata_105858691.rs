use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858691: FileFormat = FileFormat {
    id: 105_858_691,
    puid: "wikidata/105858691",
    name: "Berkeley Logic Interchange Format",
    extensions: &["blif"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x6D, 0x6F, 0x64, 0x65, 0x6C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
