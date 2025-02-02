use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851577: FileFormat = FileFormat {
    id: 105_851_577,
    source_type: SourceType::Wikidata,
    name: "DVD TEXT data file",
    extensions: &["txtdt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x56, 0x44, 0x5F, 0x54, 0x45, 0x58, 0x54, 0x5F, 0x44, 0x41, 0x54, 0x41,
                    0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
