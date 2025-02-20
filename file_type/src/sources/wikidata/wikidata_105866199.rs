use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866199: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_199,
        source_type: SourceType::Wikidata,
        name: "Floppy Disk Emulator configuration",
        extensions: &["par"],
        media_types: &["text/play"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x52, 0x56, 0x5F, 0x53, 0x45, 0x4C, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
