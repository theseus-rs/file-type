use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205832: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_832,
        source_type: SourceType::Wikidata,
        name: "Cisco IP Phone image",
        extensions: &["cip"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x43, 0x69, 0x73, 0x63, 0x6F, 0x49, 0x50, 0x50, 0x68, 0x6F, 0x6E,
                        0x65, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
