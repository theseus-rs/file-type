use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4047266: FileFormat = FileFormat {
    id: 4_047_266,
    puid: "wikidata/4047266",
    name: "Portable Draughts Notation",
    extensions: &["pdn"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
