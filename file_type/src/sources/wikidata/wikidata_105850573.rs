use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850573: FileFormat = FileFormat {
    id: 105_850_573,
    source_type: SourceType::Wikidata,
    name: "Compaq Diagnostics",
    extensions: &["cva"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x43, 0x56, 0x41, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x49, 0x6E, 0x66,
                    0x6F, 0x72, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
