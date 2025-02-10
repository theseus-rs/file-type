use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867352: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_352,
        source_type: SourceType::Wikidata,
        name: "myBase database",
        extensions: &["nyf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x75, 0x54, 0x31, 0x01, 0x01, 0x80, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
