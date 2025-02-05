use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854954: FileFormat = FileFormat {
    id: 105_854_954,
    source_type: SourceType::Wikidata,
    name: "Cheetah subtitles",
    extensions: &["asc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x4E, 0x6F, 0x6E, 0x44, 0x72, 0x6F, 0x70, 0x46, 0x72, 0x61, 0x6D, 0x65,
                    0x0D, 0x0A, 0x2A, 0x57, 0x69, 0x64, 0x74, 0x68,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
