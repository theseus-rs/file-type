use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857206: FileFormat = FileFormat {
    id: 105_857_206,
    puid: "wikidata/105857206",
    name: "High Voltage SID Collection update info",
    extensions: &["hvs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x48, 0x69, 0x67, 0x68, 0x20, 0x56, 0x6F, 0x6C, 0x74, 0x61, 0x67,
                    0x65, 0x20, 0x53, 0x49, 0x44, 0x20, 0x43, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74,
                    0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
