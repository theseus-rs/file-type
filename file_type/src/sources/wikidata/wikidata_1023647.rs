use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_1023647: FileType = FileType {
    file_format: &FileFormat {
        id: 1_023_647,
        source_type: SourceType::Wikidata,
        name: "Microsoft Compiled HTML Help",
        extensions: &["chm"],
        media_types: &["application/vnd.ms-htmlhelp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x54, 0x53, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
