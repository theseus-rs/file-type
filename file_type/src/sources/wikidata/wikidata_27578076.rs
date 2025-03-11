use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27578076: FileType = FileType {
    file_format: &FileFormat {
        id: 27_578_076,
        source_type: SourceType::Wikidata,
        name: "AppleDouble Resource Fork, version 1",
        extensions: &[],
        media_types: &["multipart/appledouble"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x05, 0x16, 0x07, 0x00, 0x01, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
