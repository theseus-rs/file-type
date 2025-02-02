use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979385: FileFormat = FileFormat {
    id: 27_979_385,
    source_type: SourceType::Wikidata,
    name: "Fractal Image And Sequence Codec",
    extensions: &["fco"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x41, 0x53, 0x43, 0x4F, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
