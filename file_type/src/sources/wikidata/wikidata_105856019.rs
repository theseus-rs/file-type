use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856019: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_019,
        source_type: SourceType::Wikidata,
        name: "Dynojet Run File",
        extensions: &["drf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x48, 0x37, 0x30, 0x33, 0x0D, 0x0A, 0x44, 0x79, 0x6E, 0x6F, 0x6A,
                        0x65, 0x74, 0x20, 0x52, 0x65, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20,
                        0x32, 0x30, 0x30, 0x32, 0x0D, 0x0A, 0x1A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
