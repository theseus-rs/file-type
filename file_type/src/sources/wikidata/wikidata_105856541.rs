use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856541: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_541,
        source_type: SourceType::Wikidata,
        name: "AIM Extended Wavefunction",
        extensions: &["wfx"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3E, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
