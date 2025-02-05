use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860524: FileFormat = FileFormat {
    id: 105_860_524,
    source_type: SourceType::Wikidata,
    name: "RAR Password Cracker project",
    extensions: &["rpc"],
    media_types: &[],
    signatures: &[Signature {
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
