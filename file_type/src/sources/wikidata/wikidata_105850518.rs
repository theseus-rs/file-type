use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850518: FileFormat = FileFormat {
    id: 105_850_518,
    source_type: SourceType::Wikidata,
    name: "CadStd drawing",
    extensions: &["cad"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x44, 0x53, 0x54, 0x44, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
