use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858049: FileFormat = FileFormat {
    id: 105_858_049,
    puid: "wikidata/105858049",
    name: "Infinity Engine Area (v9.1)",
    extensions: &["are"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x52, 0x45, 0x41, 0x56, 0x39, 0x2E, 0x31, 0x41, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
