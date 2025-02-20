use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856218: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_218,
        source_type: SourceType::Wikidata,
        name: "DeepBurner project",
        extensions: &["dbr"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x44, 0x65, 0x65, 0x70, 0x42, 0x75, 0x72, 0x6E, 0x65, 0x72, 0x5F,
                        0x72, 0x65, 0x63, 0x6F, 0x72, 0x64, 0x20, 0x76, 0x65, 0x72, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
