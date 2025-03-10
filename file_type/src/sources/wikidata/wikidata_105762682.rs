use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762682: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_682,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel for OS/2 worksheet (v.unk)",
        extensions: &[],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x09, 0x00, 0x04, 0x00, 0x05, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
