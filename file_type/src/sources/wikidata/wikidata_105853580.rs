use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853580: FileFormat = FileFormat {
    id: 105_853_580,
    source_type: SourceType::Wikidata,
    name: "ZenPhoto Database Backup",
    extensions: &["zdb"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x5F, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x5F, 0x5F, 0x66, 0x69, 0x6C,
                    0x65, 0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
