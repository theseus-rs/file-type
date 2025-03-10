use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_784,
        source_type: SourceType::Wikidata,
        name: "Varian UV-VIS-NIR specral data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x11, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6E, 0x20, 0x55, 0x56, 0x2D, 0x56,
                        0x49, 0x53, 0x2D, 0x4E, 0x49, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
