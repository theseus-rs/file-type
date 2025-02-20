use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856198: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_198,
        source_type: SourceType::Wikidata,
        name: "DipTrace Schematic",
        extensions: &["dch"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x07, 0x44, 0x54, 0x53, 0x43, 0x48, 0x45, 0x4D, 0x0F, 0x42,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
