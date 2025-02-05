use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857673: FileFormat = FileFormat {
    id: 105_857_673,
    source_type: SourceType::Wikidata,
    name: "ACT! Internet Mail message",
    extensions: &["iml"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x87, 0x2A, 0x02, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
