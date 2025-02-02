use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859971: FileFormat = FileFormat {
    id: 105_859_971,
    source_type: SourceType::Wikidata,
    name: "Fast Search and Transfer video",
    extensions: &["fvt"],
    media_types: &["video/vnd.fvt"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x56, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
