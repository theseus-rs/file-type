use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852150: FileFormat = FileFormat {
    id: 105_852_150,
    source_type: SourceType::Wikidata,
    name: "IBM PC Storyboard Story",
    extensions: &["sh~"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x50, 0x5F, 0x53, 0x48, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
