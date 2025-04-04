use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853820: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_820,
        source_type: SourceType::Wikidata,
        name: "Advanced Installer Updates configuration",
        extensions: &["aiu"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B, 0x61, 0x69, 0x75, 0x3B, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
