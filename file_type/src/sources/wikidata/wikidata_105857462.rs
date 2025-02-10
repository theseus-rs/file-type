use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857462: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_462,
        source_type: SourceType::Wikidata,
        name: "2IMG Universal Format disk image (Apple II)",
        extensions: &["2img", "2mg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x49, 0x4D, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
