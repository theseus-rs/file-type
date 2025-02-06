use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852367: FileFormat = FileFormat {
    id: 105_852_367,
    source_type: SourceType::Wikidata,
    name: "ABC SnapGraphix Graph",
    extensions: &["sg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x1D, 0x00, 0x47, 0x46, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
