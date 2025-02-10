use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28975835: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_835,
        source_type: SourceType::Wikidata,
        name: "MilkShape 3D",
        extensions: &["ms3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x33, 0x44, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
