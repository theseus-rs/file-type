use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856784: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_784,
        source_type: SourceType::Wikidata,
        name: "Microsoft Exchange Server Gather log",
        extensions: &["gthr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x53, 0x65,
                        0x61, 0x72, 0x63, 0x68, 0x20, 0x47, 0x61, 0x74, 0x68, 0x65, 0x72, 0x65,
                        0x72, 0x20, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6F,
                        0x6E, 0x20, 0x4C, 0x6F, 0x67, 0x2E, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61,
                        0x74, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
