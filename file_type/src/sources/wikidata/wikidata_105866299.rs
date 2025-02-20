use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866299: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_299,
        source_type: SourceType::Wikidata,
        name: "Parity Archive Volume Set (Par1)",
        extensions: &["par"],
        media_types: &["application/x-par"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x41, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
