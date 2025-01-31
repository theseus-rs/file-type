use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862379: FileFormat = FileFormat {
    id: 105_862_379,
    puid: "wikidata/105862379",
    name: "Personal Font Maker Macro",
    extensions: &["mcr"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x46, 0x4D, 0x20, 0x4D, 0x43, 0x52, 0x4F, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
