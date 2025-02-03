use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866055: FileFormat = FileFormat {
    id: 27_866_055,
    source_type: SourceType::Wikidata,
    name: "bzip Archive",
    extensions: &["bz"],
    media_types: &["application/x-bzip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x5A, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
