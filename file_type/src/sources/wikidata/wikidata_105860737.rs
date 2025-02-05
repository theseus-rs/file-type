use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860737: FileFormat = FileFormat {
    id: 105_860_737,
    source_type: SourceType::Wikidata,
    name: "Rapport Script",
    extensions: &["rsp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x5D, 0x0D, 0x0A, 0x4E, 0x75,
                    0x6D, 0x62, 0x65, 0x72, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
