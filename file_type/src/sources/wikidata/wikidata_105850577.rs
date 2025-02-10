use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850577: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_577,
        source_type: SourceType::Wikidata,
        name: "CorelDRAW for OS/2 drawing (v2.5)",
        extensions: &["cdr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x4C, 0xDC, 0x00, 0x25, 0x05])],
                },
            }],
        }],
        related_formats: &[],
    },
};
