use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852162: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_162,
        source_type: SourceType::Wikidata,
        name: "Lego Loco save game",
        extensions: &["sav"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x08, 0x00, 0x40, 0x00, 0x30, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
