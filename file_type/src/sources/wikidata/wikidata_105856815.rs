use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856815: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_815,
        source_type: SourceType::Wikidata,
        name: "Jinxter game save (PC)",
        extensions: &["gam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x77, 0x6A, 0x50, 0x61, 0x88, 0x91, 0x50, 0x61, 0x77, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
