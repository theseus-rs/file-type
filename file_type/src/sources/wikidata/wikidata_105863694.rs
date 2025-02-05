use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863694: FileFormat = FileFormat {
    id: 105_863_694,
    source_type: SourceType::Wikidata,
    name: "MUI Builder project",
    extensions: &["muib"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x55, 0x49, 0x4C, 0x44, 0x45, 0x52, 0x5F, 0x53, 0x41, 0x56, 0x45, 0x5F,
                    0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
