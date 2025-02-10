use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27526137: FileType = FileType {
    file_format: &FileFormat {
        id: 27_526_137,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for Macintosh Document, version 3.0",
        extensions: &["mcw"],
        media_types: &["application/msword"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x34, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
