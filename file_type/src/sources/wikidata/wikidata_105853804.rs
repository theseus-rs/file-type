use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853804: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_804,
        source_type: SourceType::Wikidata,
        name: "SuperDOS Adapter File",
        extensions: &["adf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x64, 0x61, 0x70, 0x74, 0x65, 0x72, 0x49, 0x64, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
