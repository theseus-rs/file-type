use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854257: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_257,
        source_type: SourceType::Wikidata,
        name: "Musepack encoded audio (SV7.0)",
        extensions: &["mpc"],
        media_types: &["audio/musepack"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x2B, 0x07])],
                },
            }],
        }],
        related_formats: &[],
    },
};
