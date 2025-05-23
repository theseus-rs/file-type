use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861569: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_569,
        source_type: SourceType::Wikidata,
        name: "Leggless Music Editor module",
        extensions: &["lme"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x4D, 0x45, 0x00, 0x28, 0x63, 0x29, 0x31, 0x39, 0x39, 0x30, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
