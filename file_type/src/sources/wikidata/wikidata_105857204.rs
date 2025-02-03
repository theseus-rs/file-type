use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857204: FileFormat = FileFormat {
    id: 105_857_204,
    source_type: SourceType::Wikidata,
    name: "HandStory eBook",
    extensions: &["hsb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x01, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
