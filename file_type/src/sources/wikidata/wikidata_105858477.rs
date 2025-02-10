use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858477: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_477,
        source_type: SourceType::Wikidata,
        name: "Emergency 4 ModPack",
        extensions: &["e4mod"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x34, 0x4D, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
