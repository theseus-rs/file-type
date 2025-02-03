use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861724: FileFormat = FileFormat {
    id: 105_861_724,
    source_type: SourceType::Wikidata,
    name: "Multiple Sequence File (amino acid)",
    extensions: &["msf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x21, 0x41, 0x41, 0x5F, 0x4D, 0x55, 0x4C, 0x54, 0x49, 0x50, 0x4C, 0x45,
                    0x5F, 0x41, 0x4C, 0x49, 0x47, 0x4E, 0x4D, 0x45, 0x4E, 0x54, 0x20, 0x31, 0x2E,
                    0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
