use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851232: FileFormat = FileFormat {
    id: 105_851_232,
    source_type: SourceType::Wikidata,
    name: "Compressed TYPO3 extension",
    extensions: &["t3x"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3A, 0x67, 0x7A, 0x63, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x3A, 0x78,
                    0x9C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
