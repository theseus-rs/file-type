use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859930: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_930,
        source_type: SourceType::Wikidata,
        name: "YUV4MPEG2 video",
        extensions: &["y4m"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x59, 0x55, 0x56, 0x34, 0x4D, 0x50, 0x45, 0x47, 0x32, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
