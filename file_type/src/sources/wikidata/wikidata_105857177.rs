use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857177: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_177,
        source_type: SourceType::Wikidata,
        name: "HxC Floppy Emulator stream image",
        extensions: &["hfe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x78, 0x43, 0x5F, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x5F, 0x49,
                        0x6D, 0x61, 0x67, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
