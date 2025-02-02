use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867326: FileFormat = FileFormat {
    id: 105_867_326,
    source_type: SourceType::Wikidata,
    name: "Nestopia savestate",
    extensions: &["nst"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x53, 0x54, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
