use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852282: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_282,
        source_type: SourceType::Wikidata,
        name: "SUNTronic module",
        extensions: &["sun"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0xE7, 0xFF, 0xFE, 0x4D, 0xFA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
