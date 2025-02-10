use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856592: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_592,
        source_type: SourceType::Wikidata,
        name: "Wired For Sound configuration",
        extensions: &["wfs"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x57, 0x46, 0x53, 0x6F, 0x75, 0x6E, 0x64, 0x5D, 0x0D, 0x0A, 0x56,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x32, 0x0D, 0x0A, 0x0D, 0x0A,
                        0x5B, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
