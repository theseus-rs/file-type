use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855613: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_613,
        source_type: SourceType::Wikidata,
        name: "OpenStreetMap O5c data",
        extensions: &["o5c"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xE0, 0x04, 0x6F, 0x35, 0x63, 0x32, 0xDB, 0x12,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
