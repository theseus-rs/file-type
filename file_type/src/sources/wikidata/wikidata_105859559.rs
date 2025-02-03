use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859559: FileFormat = FileFormat {
    id: 105_859_559,
    source_type: SourceType::Wikidata,
    name: "A-10 Tank Killer game data archive",
    extensions: &["vol"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x4F, 0x4C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
