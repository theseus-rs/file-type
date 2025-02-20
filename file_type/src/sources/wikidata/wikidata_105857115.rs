use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857115: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_115,
        source_type: SourceType::Wikidata,
        name: "FL Studio GM Synth",
        extensions: &["gmsynth"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x6E, 0x79, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
