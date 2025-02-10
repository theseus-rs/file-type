use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205633: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_633,
        source_type: SourceType::Wikidata,
        name: "SuperJPG thumbnail cache",
        extensions: &["tnc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0x00, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
