use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856609: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_609,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works for DOS Spreadsheet",
        extensions: &["wks"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x02, 0x00, 0x04, 0x04, 0x05, 0x54, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
