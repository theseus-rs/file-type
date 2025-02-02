use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864278: FileFormat = FileFormat {
    id: 105_864_278,
    source_type: SourceType::Wikidata,
    name: "World Construction Set Project",
    extensions: &["proj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x43, 0x53, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
