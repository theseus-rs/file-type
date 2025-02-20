use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850350: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_350,
        source_type: SourceType::Wikidata,
        name: "Clonk game data",
        extensions: &["c4*"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1E, 0x8C, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0xEC,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
