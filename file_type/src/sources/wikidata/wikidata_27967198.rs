use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967198: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_198,
        source_type: SourceType::Wikidata,
        name: "Liquid Digitized Sample",
        extensions: &["lds"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x44, 0x53, 0x53, 0x01, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
