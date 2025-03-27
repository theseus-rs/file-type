use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1064039: FileType = FileType {
    file_format: &FileFormat {
        id: 1_064_039,
        source_type: SourceType::Wikidata,
        name: "JPEG XR",
        extensions: &["hdp", "jxr", "wdp", "wmp"],
        media_types: &["image/jxr", "image/vnd.ms-photo"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0xBC])],
                },
            }],
        }],
        related_formats: &[],
    },
};
