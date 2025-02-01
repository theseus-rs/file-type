use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857484: FileFormat = FileFormat {
    id: 105_857_484,
    puid: "wikidata/105857484",
    name: "SCUMM main data container (v6)",
    extensions: &["001"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x45, 0x43, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
