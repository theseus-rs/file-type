use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919058: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_058,
        source_type: SourceType::Wikidata,
        name: "Adobe Premiere Title",
        extensions: &["ptl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x54, 0x54, 0x4C, 0x35, 0x5D, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
