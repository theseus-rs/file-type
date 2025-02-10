use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866642: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_642,
        source_type: SourceType::Wikidata,
        name: "Programmer's Notepad State",
        extensions: &["pnps"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x70, 0x64, 0x3E, 0x3C, 0x56, 0x69, 0x65, 0x77, 0x53, 0x74, 0x61,
                        0x74, 0x65, 0x3E, 0x3C, 0x65, 0x20, 0x70, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
