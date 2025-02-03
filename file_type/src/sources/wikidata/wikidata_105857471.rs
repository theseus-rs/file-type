use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857471: FileFormat = FileFormat {
    id: 105_857_471,
    source_type: SourceType::Wikidata,
    name: "010 Editor Template List",
    extensions: &["1tl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x31, 0x54, 0x4C, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
