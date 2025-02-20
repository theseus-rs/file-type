use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867331: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_331,
        source_type: SourceType::Wikidata,
        name: "NanoZoomer Annotation",
        extensions: &["ndpa"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x6E, 0x64, 0x70, 0x76, 0x69, 0x65, 0x77, 0x73, 0x74, 0x61, 0x74,
                        0x65, 0x20, 0x69, 0x64, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
