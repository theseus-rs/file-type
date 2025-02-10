use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857650: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_650,
        source_type: SourceType::Wikidata,
        name: "PaintTool SAI Tool parameters",
        extensions: &["ini"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x54, 0x6F, 0x6F, 0x6C, 0x50, 0x61, 0x72, 0x61, 0x6D, 0x5D, 0x0D,
                        0x0A, 0x69, 0x64, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
