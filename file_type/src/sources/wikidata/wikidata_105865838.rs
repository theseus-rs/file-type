use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865838: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_838,
        source_type: SourceType::Wikidata,
        name: "Previous emulator configuration",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x4C, 0x6F, 0x67, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
