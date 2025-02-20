use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856241: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_241,
        source_type: SourceType::Wikidata,
        name: "Sonic Adventure DX game data archive",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x20, 0x20, 0x56, 0x32, 0x2E,
                        0x32, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
