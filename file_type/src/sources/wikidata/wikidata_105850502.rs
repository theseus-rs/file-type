use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850502: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_502,
        source_type: SourceType::Wikidata,
        name: "OCaml bytecode (library)",
        extensions: &["cma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x61, 0x6D, 0x6C, 0x31, 0x39, 0x39, 0x39, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
