use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857676: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_676,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine Animation (v1)",
        extensions: &["bam"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x41, 0x4D, 0x20, 0x56, 0x31, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
