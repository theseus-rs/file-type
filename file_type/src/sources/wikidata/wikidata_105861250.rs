use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861250: FileFormat = FileFormat {
    id: 105_861_250,
    source_type: SourceType::Wikidata,
    name: "LogonStudio Vista logon image",
    extensions: &["logonvista"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x56, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
