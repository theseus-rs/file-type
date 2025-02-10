use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862906: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_906,
        source_type: SourceType::Wikidata,
        name: "Mirror II Emulation File",
        extensions: &["mef"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x0E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
