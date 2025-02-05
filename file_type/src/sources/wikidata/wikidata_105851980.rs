use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851980: FileFormat = FileFormat {
    id: 105_851_980,
    source_type: SourceType::Wikidata,
    name: "Windows Setup Table File",
    extensions: &["stf"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x70, 0x70, 0x20, 0x4E, 0x61, 0x6D, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
