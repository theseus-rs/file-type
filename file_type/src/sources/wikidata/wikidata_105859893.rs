use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859893: FileFormat = FileFormat {
    id: 105_859_893,
    source_type: SourceType::Wikidata,
    name: "Micro Fly Movie Format video",
    extensions: &["ufmf"],
    media_types: &["video/ufmf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x75, 0x66, 0x6D, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
