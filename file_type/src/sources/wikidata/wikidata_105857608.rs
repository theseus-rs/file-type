use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857608: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_608,
        source_type: SourceType::Wikidata,
        name: "TraxPlayer audio floppy image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEB, 0xFE, 0x90, 0x54, 0x72, 0x61, 0x78, 0x50, 0x6C, 0x61, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
