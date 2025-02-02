use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862277: FileFormat = FileFormat {
    id: 105_862_277,
    source_type: SourceType::Wikidata,
    name: "Room Arranger design",
    extensions: &["mst"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x52, 0x6F, 0x6F, 0x6D, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
