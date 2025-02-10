use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27959996: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_996,
        source_type: SourceType::Wikidata,
        name: "OptimFROG",
        extensions: &["ofr", "ofs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x46, 0x52, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
