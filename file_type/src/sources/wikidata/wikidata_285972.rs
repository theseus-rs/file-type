use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_285972: FileFormat = FileFormat {
    id: 285_972,
    source_type: SourceType::Wikidata,
    name: "Mbox",
    extensions: &["mbox", "mbx"],
    media_types: &["application/mbox"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x72, 0x6F, 0x6D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
