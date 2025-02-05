use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856852: FileFormat = FileFormat {
    id: 105_856_852,
    source_type: SourceType::Wikidata,
    name: "Genstat Theme",
    extensions: &["gth"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x47, 0x65, 0x6E, 0x73, 0x74, 0x61, 0x74, 0x20, 0x54, 0x68, 0x65, 0x6D,
                    0x65, 0x5D, 0x0D, 0x0A, 0x54, 0x68, 0x65, 0x6D, 0x65, 0x4E, 0x61, 0x6D, 0x65,
                    0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
