use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858386: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_386,
        source_type: SourceType::Wikidata,
        name: "E-mu Emulator III sample",
        extensions: &["ez3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4D, 0x55, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
