use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860016: FileFormat = FileFormat {
    id: 105_860_016,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts TGV video",
    extensions: &["tgv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6B, 0x56, 0x47, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
