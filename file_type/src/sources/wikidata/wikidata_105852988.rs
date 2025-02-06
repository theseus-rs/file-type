use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852988: FileFormat = FileFormat {
    id: 105_852_988,
    source_type: SourceType::Wikidata,
    name: "3DSMax STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x4C, 0x45, 0x58, 0x50, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
