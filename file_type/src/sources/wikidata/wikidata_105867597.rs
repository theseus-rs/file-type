use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867597: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_597,
        source_type: SourceType::Wikidata,
        name: "Nero Scalable Audio",
        extensions: &["nsla"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4E, 0x65, 0x72, 0x6F, 0x53, 0x63, 0x61, 0x6C, 0x61, 0x62, 0x6C,
                        0x65, 0x41, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
