use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762772: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_772,
        source_type: SourceType::Wikidata,
        name: "Xilinx internal data",
        extensions: &[],
        media_types: &["application/vnd.xilinx.internal"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x6C, 0x78, 0x56, 0x33, 0x37, 0x45, 0x42, 0x20, 0x20, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
