use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853103: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_103,
        source_type: SourceType::Wikidata,
        name: "ShowMaker Device",
        extensions: &["smdevice"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x45, 0x56, 0x49, 0x43, 0x45, 0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
