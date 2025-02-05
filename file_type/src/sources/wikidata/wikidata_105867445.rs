use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867445: FileFormat = FileFormat {
    id: 105_867_445,
    source_type: SourceType::Wikidata,
    name: "GraalOnline level",
    extensions: &["nw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4C, 0x45, 0x56, 0x4E, 0x57, 0x30, 0x31, 0x0A, 0x42, 0x4F, 0x41, 0x52,
                    0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
