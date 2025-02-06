use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979504: FileFormat = FileFormat {
    id: 27_979_504,
    source_type: SourceType::Wikidata,
    name: "GIMP Palette",
    extensions: &["gpl"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x49, 0x4D, 0x50, 0x20, 0x50, 0x61, 0x6C, 0x65, 0x74, 0x74, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
