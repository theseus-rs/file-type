use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205498: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_498,
        source_type: SourceType::Wikidata,
        name: "EPOC Application Information File",
        extensions: &["aif"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x00, 0x00, 0x10, 0x6A, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
