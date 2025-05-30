use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853174: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_174,
        source_type: SourceType::Wikidata,
        name: "Meshmixer STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x45, 0x53, 0x48, 0x4D, 0x49, 0x58, 0x45, 0x52, 0x2D, 0x53, 0x54,
                        0x4C, 0x2D, 0x42, 0x49, 0x4E, 0x41, 0x52, 0x59, 0x2D, 0x46, 0x4F, 0x52,
                        0x4D, 0x41, 0x54, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
