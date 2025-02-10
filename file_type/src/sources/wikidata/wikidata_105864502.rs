use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864502: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_502,
        source_type: SourceType::Wikidata,
        name: "Portable Heap Dump dump",
        extensions: &["phd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x12, 0x70, 0x6F, 0x72, 0x74, 0x61, 0x62, 0x6C, 0x65, 0x20, 0x68,
                        0x65, 0x61, 0x70, 0x20, 0x64, 0x75, 0x6D, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
