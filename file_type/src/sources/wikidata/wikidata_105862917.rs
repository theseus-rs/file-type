use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862917: FileFormat = FileFormat {
    id: 105_862_917,
    puid: "wikidata/105862917",
    name: "NeXtMidas Macro",
    extensions: &["mm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x74, 0x61, 0x72, 0x74, 0x6D, 0x61, 0x63, 0x72, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
