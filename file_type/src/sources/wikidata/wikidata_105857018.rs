use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857018: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_018,
        source_type: SourceType::Wikidata,
        name: "Microsoft Vista Sidebar Gadget (CAB - Obsolete)",
        extensions: &["gadget"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x43, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
