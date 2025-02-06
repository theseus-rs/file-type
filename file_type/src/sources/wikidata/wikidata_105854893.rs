use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854893: FileFormat = FileFormat {
    id: 105_854_893,
    source_type: SourceType::Wikidata,
    name: "Interfaze Application",
    extensions: &["app"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x50, 0x43, 0x52, 0xA8, 0xAF, 0xBC, 0xAD,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
