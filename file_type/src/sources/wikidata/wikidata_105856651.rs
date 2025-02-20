use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856651: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_651,
        source_type: SourceType::Wikidata,
        name: "Tektronix TDS extended waveform data",
        extensions: &["wfm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4C, 0x57, 0x46, 0x31, 0x20, 0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
