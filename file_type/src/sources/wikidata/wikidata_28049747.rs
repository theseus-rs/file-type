use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28049747: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_747,
        source_type: SourceType::Wikidata,
        name: "CMU Andrew Toolkit image",
        extensions: &["atk", "cmu"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x62, 0x65, 0x67, 0x69, 0x6E, 0x64, 0x61, 0x74, 0x61, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
