use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_45347388: FileType = FileType {
    file_format: &FileFormat {
        id: 45_347_388,
        source_type: SourceType::Wikidata,
        name: "Lotus 1-2-3 Worksheet file format, version 3",
        extensions: &["wk3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x1A, 0x00, 0x00, 0x10, 0x04, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
