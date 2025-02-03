use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966979: FileFormat = FileFormat {
    id: 27_966_979,
    source_type: SourceType::Wikidata,
    name: "Organya",
    extensions: &["org"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x72, 0x67, 0x2D, 0x30, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
