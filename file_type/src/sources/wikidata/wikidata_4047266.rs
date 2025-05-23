use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4047266: FileType = FileType {
    file_format: &FileFormat {
        id: 4_047_266,
        source_type: SourceType::Wikidata,
        name: "Portable Draughts Notation",
        extensions: &["pdn"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x45, 0x76, 0x65, 0x6E, 0x74, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
