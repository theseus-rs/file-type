use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855744: FileFormat = FileFormat {
    id: 105_855_744,
    source_type: SourceType::Wikidata,
    name: "Dream Station 2.0 module",
    extensions: &["ds2"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x32, 0x46, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
