use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_47245245: FileType = FileType {
    file_format: &FileFormat {
        id: 47_245_245,
        source_type: SourceType::Wikidata,
        name: "Microsoft Network Monitor Packet Capture, version 1",
        extensions: &["cap"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x54, 0x53, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
