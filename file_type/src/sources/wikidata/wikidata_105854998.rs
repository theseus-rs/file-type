use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854998: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_998,
        source_type: SourceType::Wikidata,
        name: "Musepack encoded audio (SV8)",
        extensions: &["mpc"],
        media_types: &["audio/musepack"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x43, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
