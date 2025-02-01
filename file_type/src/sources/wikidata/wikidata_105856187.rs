use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856187: FileFormat = FileFormat {
    id: 105_856_187,
    puid: "wikidata/105856187",
    name: "ClrMamePro DAT / MAME Listinfo format",
    extensions: &["dat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6C, 0x72, 0x6D, 0x61, 0x6D, 0x65, 0x70, 0x72, 0x6F, 0x20, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
