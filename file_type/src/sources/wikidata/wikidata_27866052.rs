use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866052: FileFormat = FileFormat {
    id: 27_866_052,
    source_type: SourceType::Wikidata,
    name: "bzip2 Archive",
    extensions: &["bz2"],
    media_types: &["application/x-bzip2"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x5A, 0x68])],
            },
        }],
    }],
    related_formats: &[],
};
