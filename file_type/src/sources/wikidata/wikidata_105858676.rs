use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858676: FileFormat = FileFormat {
    id: 105_858_676,
    source_type: SourceType::Wikidata,
    name: "ColorViewSquash bitmap",
    extensions: &["rgb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x47, 0x42, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
