use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852989: FileFormat = FileFormat {
    id: 105_852_989,
    source_type: SourceType::Wikidata,
    name: "Squeeze project",
    extensions: &["sqz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x73, 0x71, 0x75,
                    0x65, 0x65, 0x7A, 0x65, 0x5F, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
