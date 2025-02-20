use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859585: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_585,
        source_type: SourceType::Wikidata,
        name: "PlayStation single waveform data format",
        extensions: &["vag"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x41, 0x47, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
