use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853611: FileFormat = FileFormat {
    id: 105_853_611,
    source_type: SourceType::Wikidata,
    name: "AlterImage Font",
    extensions: &["aif"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4D, 0x46, 0x4D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
