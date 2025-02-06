use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861912: FileFormat = FileFormat {
    id: 105_861_912,
    source_type: SourceType::Wikidata,
    name: "STEP7-Micro WIN PLC Program",
    extensions: &["mwp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4A, 0x4B, 0x00, 0x52, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
