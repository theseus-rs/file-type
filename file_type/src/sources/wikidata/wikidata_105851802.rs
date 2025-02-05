use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851802: FileFormat = FileFormat {
    id: 105_851_802,
    source_type: SourceType::Wikidata,
    name: "Geomagic Studio STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x54, 0x4C, 0x20, 0x4F, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x66, 0x72,
                    0x6F, 0x6D, 0x20, 0x67, 0x65, 0x6F, 0x6D, 0x61, 0x67, 0x69, 0x63, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
