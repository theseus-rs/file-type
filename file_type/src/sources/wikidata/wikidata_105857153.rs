use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857153: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_153,
        source_type: SourceType::Wikidata,
        name: "HPub HTML Publication",
        extensions: &["hpub"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
