use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975655: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_655,
        source_type: SourceType::Wikidata,
        name: "RenderMan Interface Bytestream",
        extensions: &["rib"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x23, 0x52, 0x65, 0x6E, 0x64, 0x65, 0x72, 0x4D, 0x61, 0x6E, 0x20,
                        0x52, 0x49, 0x42,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
