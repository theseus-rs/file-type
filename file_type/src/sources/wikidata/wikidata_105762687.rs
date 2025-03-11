use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762687: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_687,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel for OS/2 worksheet (v3.x)",
        extensions: &[],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x09, 0x02, 0x06, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
