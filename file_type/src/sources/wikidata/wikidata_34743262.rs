use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_34743262: FileType = FileType {
    file_format: &FileFormat {
        id: 34_743_262,
        source_type: SourceType::Wikidata,
        name: "Softdisk Text Compressor",
        extensions: &["ctx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x43, 0x54, 0x30, 0x30, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
