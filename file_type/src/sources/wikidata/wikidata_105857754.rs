use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857754: FileFormat = FileFormat {
    id: 105_857_754,
    puid: "wikidata/105857754",
    name: "Infinity Engine archive (v1)",
    extensions: &["bif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x46, 0x46, 0x56, 0x31, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
