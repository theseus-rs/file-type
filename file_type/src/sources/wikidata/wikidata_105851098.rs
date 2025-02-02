use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851098: FileFormat = FileFormat {
    id: 105_851_098,
    source_type: SourceType::Wikidata,
    name: "LaTeX 2e document",
    extensions: &["tex"],
    media_types: &["application/x-tex"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x63, 0x6C, 0x61, 0x73,
                    0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
