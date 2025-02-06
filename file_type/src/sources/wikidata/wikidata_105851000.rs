use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851000: FileFormat = FileFormat {
    id: 105_851_000,
    source_type: SourceType::Wikidata,
    name: "Torque audio asset (XML)",
    extensions: &["taml"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x41, 0x75, 0x64, 0x69, 0x6F, 0x41, 0x73, 0x73, 0x65, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
