use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_614,
        source_type: SourceType::Wikidata,
        name: "Power2Go Image",
        extensions: &["p2i"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x61, 0x6B, 0x65, 0x50, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x74, 0x69,
                        0x6F, 0x6E, 0x54, 0x79, 0x70, 0x65, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
