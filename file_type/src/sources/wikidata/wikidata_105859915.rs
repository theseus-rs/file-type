use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859915: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_915,
        source_type: SourceType::Wikidata,
        name: "Volition Package - Red Faction game data archive",
        extensions: &["vpp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xCE, 0x0A, 0x89, 0x51, 0x01, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
