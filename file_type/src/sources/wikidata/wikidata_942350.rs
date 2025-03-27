use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_942350: FileType = FileType {
    file_format: &FileFormat {
        id: 942_350,
        source_type: SourceType::Wikidata,
        name: "QuickTime File Format",
        extensions: &["mov", "qt"],
        media_types: &["video/quicktime"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x14, 0x66, 0x74, 0x79, 0x70, 0x71, 0x74, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
