use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855831: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_831,
        source_type: SourceType::Wikidata,
        name: "AutoCAD R2.10 Drawing",
        extensions: &["dwg"],
        media_types: &["application/x-autocad"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x32, 0x2E, 0x31, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
