use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852646: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_646,
        source_type: SourceType::Wikidata,
        name: "SynthFont project",
        extensions: &["sfarr"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x46, 0x69, 0x6C, 0x65, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
