use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860245: FileFormat = FileFormat {
    id: 105_860_245,
    puid: "wikidata/105860245",
    name: "Windows Registry Data (Ver. 4.0)",
    extensions: &["reg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x45, 0x47, 0x45, 0x44, 0x49, 0x54, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
