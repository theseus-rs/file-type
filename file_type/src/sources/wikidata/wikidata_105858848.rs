use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858848: FileFormat = FileFormat {
    id: 105_858_848,
    source_type: SourceType::Wikidata,
    name: "BootSkin theme",
    extensions: &["bootskin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
