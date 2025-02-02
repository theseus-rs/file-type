use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864131: FileFormat = FileFormat {
    id: 105_864_131,
    source_type: SourceType::Wikidata,
    name: "mupen64 movie capture",
    extensions: &["m64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x36, 0x34, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
