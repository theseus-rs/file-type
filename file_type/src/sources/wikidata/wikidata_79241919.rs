use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_79241919: FileFormat = FileFormat {
    id: 79_241_919,
    source_type: SourceType::Wikidata,
    name: "ASCII Color Format",
    extensions: &["acf"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x46, 0x20, 0x31, 0x2E, 0x30, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
