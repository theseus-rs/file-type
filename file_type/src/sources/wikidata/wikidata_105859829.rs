use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859829: FileFormat = FileFormat {
    id: 105_859_829,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts MPC video",
    extensions: &["mpc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x43, 0x68])],
            },
        }],
    }],
    related_formats: &[],
};
