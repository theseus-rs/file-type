use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864544: FileFormat = FileFormat {
    id: 105_864_544,
    source_type: SourceType::Wikidata,
    name: "Sony PSP/PSVita Package (finalized/retail)",
    extensions: &["pkg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7F, 0x50, 0x4B, 0x47, 0x80, 0x00, 0x00, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
