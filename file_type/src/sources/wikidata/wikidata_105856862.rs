use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856862: FileFormat = FileFormat {
    id: 105_856_862,
    source_type: SourceType::Wikidata,
    name: "GGFileSPlit File Fragment",
    extensions: &["gfs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x47, 0x46, 0x53, 0x53, 0x74, 0x75, 0x62, 0x20, 0x5F, 0x53, 0x74, 0x75,
                    0x62, 0x46, 0x69, 0x6C, 0x65, 0x50, 0x61, 0x74, 0x68, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
