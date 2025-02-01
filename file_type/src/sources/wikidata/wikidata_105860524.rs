use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860524: FileFormat = FileFormat {
    id: 105_860_524,
    puid: "wikidata/105860524",
    name: "RAR Password Cracker project",
    extensions: &["rpc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x50, 0x43, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
