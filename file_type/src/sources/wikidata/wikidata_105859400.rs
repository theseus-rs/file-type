use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859400: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_400,
        source_type: SourceType::Wikidata,
        name: "Statler Stitcher",
        extensions: &["qli"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x43, 0x41, 0x44, 0x32, 0x44, 0x4D, 0x43, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
