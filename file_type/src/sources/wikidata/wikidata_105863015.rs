use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863015: FileFormat = FileFormat {
    id: 105_863_015,
    source_type: SourceType::Wikidata,
    name: "MaxiDesk Phone Book",
    extensions: &["book"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x48, 0x4F, 0x4E, 0x45, 0x42, 0x4F, 0x4F, 0x4B, 0x46, 0x49, 0x4C, 0x45,
                    0x0A, 0x31, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
