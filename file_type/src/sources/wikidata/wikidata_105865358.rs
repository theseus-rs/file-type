use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865358: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_358,
        source_type: SourceType::Wikidata,
        name: "MicroImages Package",
        extensions: &["pkg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x49, 0x20, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20, 0x46,
                        0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
