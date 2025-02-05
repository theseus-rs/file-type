use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865637: FileFormat = FileFormat {
    id: 105_865_637,
    source_type: SourceType::Wikidata,
    name: "Marcel Printer Driver",
    extensions: &["prd"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x4D, 0x61, 0x72, 0x63, 0x65, 0x6C, 0x20, 0x50, 0x72, 0x69, 0x6E,
                    0x74, 0x65, 0x72, 0x20, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
