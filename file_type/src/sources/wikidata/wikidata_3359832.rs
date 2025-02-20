use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3359832: FileType = FileType {
    file_format: &FileFormat {
        id: 3_359_832,
        source_type: SourceType::Wikidata,
        name: "Product Representation Compact",
        extensions: &["prc"],
        media_types: &["application/octet-stream", "model/prc"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x52, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
