use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858315: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_315,
        source_type: SourceType::Wikidata,
        name: "BitCom Emulation settings",
        extensions: &["emu"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x4E, 0x49, 0x54, 0x49, 0x41, 0x4C, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
