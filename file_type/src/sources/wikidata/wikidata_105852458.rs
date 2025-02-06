use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852458: FileFormat = FileFormat {
    id: 105_852_458,
    source_type: SourceType::Wikidata,
    name: "Opticks Scene",
    extensions: &["sce"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x53, 0x43, 0x45, 0x4E, 0x45, 0x76, 0x31, 0x2E, 0x30, 0x20, 0x7B, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
