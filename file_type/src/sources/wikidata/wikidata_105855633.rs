use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855633: FileFormat = FileFormat {
    id: 105_855_633,
    source_type: SourceType::Wikidata,
    name: "AutoPano project",
    extensions: &["oto"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x6F, 0x74, 0x6F, 0x20, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74,
                    0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74,
                    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x75, 0x74, 0x6F, 0x70, 0x61, 0x6E,
                    0x6F, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
