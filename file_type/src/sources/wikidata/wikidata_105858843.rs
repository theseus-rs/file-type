use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858843: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_843,
        source_type: SourceType::Wikidata,
        name: "KiCad pcbnew PCB layout",
        extensions: &["brd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x43, 0x42, 0x4E, 0x45, 0x57, 0x2D, 0x42, 0x4F, 0x41, 0x52, 0x44,
                        0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
