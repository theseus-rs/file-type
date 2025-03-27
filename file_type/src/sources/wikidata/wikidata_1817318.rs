use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1817318: FileType = FileType {
    file_format: &FileFormat {
        id: 1_817_318,
        source_type: SourceType::Wikidata,
        name: "X BitMap",
        extensions: &["xbm"],
        media_types: &["image/x-xbitmap"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x64, 0x65, 0x66, 0x69, 0x6E, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
