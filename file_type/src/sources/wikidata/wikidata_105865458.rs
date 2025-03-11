use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865458: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_458,
        source_type: SourceType::Wikidata,
        name: "Power 64 RAM Snapshot",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x42, 0x4D, 0x64, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
