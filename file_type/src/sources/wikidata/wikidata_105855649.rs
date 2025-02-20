use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855649: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_649,
        source_type: SourceType::Wikidata,
        name: "OrCAD Project",
        extensions: &["opj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x45, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x50, 0x72, 0x6F, 0x6A,
                        0x65, 0x63, 0x74, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
