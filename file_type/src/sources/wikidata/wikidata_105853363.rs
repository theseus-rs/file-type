use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853363: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_363,
        source_type: SourceType::Wikidata,
        name: "Snzip compressed (snappy-java format)",
        extensions: &["snappy"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x82, 0x53, 0x4E, 0x41, 0x50, 0x50, 0x59])],
                },
            }],
        }],
        related_formats: &[],
    },
};
