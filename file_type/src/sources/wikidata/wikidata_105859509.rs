use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859509: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_509,
        source_type: SourceType::Wikidata,
        name: "PVA Video (VideoStream)",
        extensions: &["pva"],
        media_types: &["video/x-pva"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x56, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
