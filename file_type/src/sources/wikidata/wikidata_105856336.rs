use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856336: FileFormat = FileFormat {
    id: 105_856_336,
    source_type: SourceType::Wikidata,
    name: "Dragon UnPACKer color Palette",
    extensions: &["dpal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x50, 0x41, 0x4C, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
