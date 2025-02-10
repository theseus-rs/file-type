use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867083: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_083,
        source_type: SourceType::Wikidata,
        name: "FreePCB Netlist",
        extensions: &["net"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x50, 0x41, 0x44, 0x53, 0x2D, 0x50, 0x43, 0x42, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
