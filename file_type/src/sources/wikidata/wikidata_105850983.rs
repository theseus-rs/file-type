use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850983: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_983,
        source_type: SourceType::Wikidata,
        name: "FOnline Engine Tile set",
        extensions: &["til"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x74, 0x69, 0x6C, 0x65, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
