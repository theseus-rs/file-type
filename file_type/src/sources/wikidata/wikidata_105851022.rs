use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851022: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_022,
        source_type: SourceType::Wikidata,
        name: "Panasonic JR Cassette image",
        extensions: &["cjr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
