use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124840545: FileFormat = FileFormat {
    id: 124_840_545,
    puid: "wikidata/124840545",
    name: "Toon Boom Project",
    extensions: &["tbp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x6F, 0x6F, 0x6E, 0x42, 0x6F, 0x6F, 0x6D, 0x54, 0x65, 0x63, 0x68, 0x6E,
                    0x6F, 0x6C, 0x6F, 0x67, 0x69, 0x65, 0x73, 0x49, 0x6E, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
