use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861879: FileFormat = FileFormat {
    id: 105_861_879,
    source_type: SourceType::Wikidata,
    name: "MaxBulk Mailer settings",
    extensions: &["mbm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x69, 0x6C, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x09,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
