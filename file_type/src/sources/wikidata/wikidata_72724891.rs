use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72724891: FileFormat = FileFormat {
    id: 72_724_891,
    source_type: SourceType::Wikidata,
    name: "C64 NBZ disk image",
    extensions: &["nbz"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4E, 0x49, 0x42, 0x2D, 0x31, 0x35, 0x34, 0x31, 0x2D, 0x52, 0x41, 0x57,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
