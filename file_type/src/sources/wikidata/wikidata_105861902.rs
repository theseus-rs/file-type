use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861902: FileFormat = FileFormat {
    id: 105_861_902,
    source_type: SourceType::Wikidata,
    name: "PlayerPro module",
    extensions: &["mad"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x44, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
