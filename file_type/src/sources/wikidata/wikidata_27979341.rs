use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979341: FileFormat = FileFormat {
    id: 27_979_341,
    source_type: SourceType::Wikidata,
    name: "Unirast",
    extensions: &["urf"],
    media_types: &["application/octet-stream", "image/urf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x4E, 0x49, 0x52, 0x41, 0x53, 0x54, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
