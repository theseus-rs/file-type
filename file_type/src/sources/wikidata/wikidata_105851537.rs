use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851537: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_537,
        source_type: SourceType::Wikidata,
        name: "TFM Music Maker music (V2)",
        extensions: &["tfe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x46, 0x4D, 0x66, 0x6D, 0x74, 0x56, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
