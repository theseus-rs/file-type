use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975805: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_805,
        source_type: SourceType::Wikidata,
        name: "AVS Field Data",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x41, 0x56, 0x53, 0x20, 0x66, 0x69, 0x65, 0x6C, 0x64, 0x20,
                        0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
