use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855648: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_648,
        source_type: SourceType::Wikidata,
        name: "EPOC OPL eXtension",
        extensions: &["opx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x79, 0x00, 0x00, 0x10, 0x5D, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
