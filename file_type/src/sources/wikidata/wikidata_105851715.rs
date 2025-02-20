use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851715: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_715,
        source_type: SourceType::Wikidata,
        name: "XnView Slideshow Data",
        extensions: &["sld"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x53, 0x6C, 0x69, 0x64, 0x65, 0x20, 0x53, 0x68, 0x6F, 0x77,
                        0x20, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6E, 0x63, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
