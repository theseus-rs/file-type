use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967391: FileFormat = FileFormat {
    id: 27_967_391,
    source_type: SourceType::Wikidata,
    name: "Adlib Tracker II instrument with fm-register macro",
    extensions: &["a2f"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x61, 0x32, 0x69, 0x6E, 0x73, 0x5F, 0x77, 0x2F, 0x66, 0x6D, 0x2D, 0x6D,
                    0x61, 0x63, 0x72, 0x6F, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
