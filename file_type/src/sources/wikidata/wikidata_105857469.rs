use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857469: FileFormat = FileFormat {
    id: 105_857_469,
    puid: "wikidata/105857469",
    name: "3Digi Parameters",
    extensions: &["3dp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x61, 0x72, 0x53, 0x65, 0x74, 0x30, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
