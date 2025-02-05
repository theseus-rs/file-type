use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858805: FileFormat = FileFormat {
    id: 105_858_805,
    source_type: SourceType::Wikidata,
    name: "Windows NTBackup archive",
    extensions: &["bkf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x41, 0x50, 0x45, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
