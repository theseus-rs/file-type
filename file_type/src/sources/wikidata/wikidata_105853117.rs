use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853117: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_117,
        source_type: SourceType::Wikidata,
        name: "Psion SH3 Spreadsheet",
        extensions: &["spr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x50, 0x52, 0x45, 0x41, 0x44, 0x53, 0x48, 0x45, 0x45, 0x54, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
