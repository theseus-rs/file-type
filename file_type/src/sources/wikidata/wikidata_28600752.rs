use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600752: FileFormat = FileFormat {
    id: 28_600_752,
    puid: "wikidata/28600752",
    name: "EGG",
    extensions: &["egg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x6F, 0x6F, 0x72, 0x64, 0x69, 0x6E, 0x61, 0x74, 0x65, 0x53, 0x79,
                    0x73, 0x74, 0x65, 0x6D, 0x3E, 0x20, 0x7B, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
