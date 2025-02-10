use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857223: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_223,
        source_type: SourceType::Wikidata,
        name: "SymbOS Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x59, 0x4D, 0x48, 0x4C, 0x50, 0x31, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
