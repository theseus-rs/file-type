use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205552: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_552,
        source_type: SourceType::Wikidata,
        name: "Nokia Group Graphic",
        extensions: &["ngg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x47, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
