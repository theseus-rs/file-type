use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851737: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_737,
        source_type: SourceType::Wikidata,
        name: "LibreOffice Gallery translations",
        extensions: &["str"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x6C, 0x61, 0x74, 0x65, 0x64,
                        0x20, 0x62, 0x79, 0x20, 0x64, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x2D,
                        0x74, 0x72, 0x61, 0x6E, 0x73, 0x6C, 0x61, 0x74, 0x65, 0x20, 0x61, 0x6E,
                        0x64, 0x20, 0x75, 0x6C, 0x66, 0x65, 0x78,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
