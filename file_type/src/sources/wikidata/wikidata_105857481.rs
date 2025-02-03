use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857481: FileFormat = FileFormat {
    id: 105_857_481,
    source_type: SourceType::Wikidata,
    name: "3-Demon project",
    extensions: &["3demon"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x4E, 0x45, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
