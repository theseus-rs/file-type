use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852425: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_425,
        source_type: SourceType::Wikidata,
        name: "SPSS Type Library",
        extensions: &["tlb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x46, 0x54, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x09, 0x04, 0x00, 0x00, 0x09, 0x04, 0x00, 0x00, 0x51, 0x00, 0x00, 0x00,
                        0x07, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
