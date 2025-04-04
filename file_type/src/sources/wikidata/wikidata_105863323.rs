use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863323: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_323,
        source_type: SourceType::Wikidata,
        name: "Quartet ST module",
        extensions: &["4q"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x55, 0x41, 0x52, 0x54, 0x45, 0x54, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
