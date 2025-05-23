use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853108: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_108,
        source_type: SourceType::Wikidata,
        name: "Winamp Signal Processing Studio DSP-Effect",
        extensions: &["sps"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x53, 0x50, 0x53, 0x20, 0x50, 0x52, 0x45, 0x53, 0x45, 0x54, 0x5D,
                        0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
