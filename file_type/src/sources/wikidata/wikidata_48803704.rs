use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48803704: FileFormat = FileFormat {
    id: 48_803_704,
    source_type: SourceType::Wikidata,
    name: "AutoSketch Drawing",
    extensions: &["skf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x53, 0x6B, 0x65, 0x74, 0x63, 0x68, 0x20, 0x64, 0x72,
                    0x61, 0x77, 0x69, 0x6E, 0x67, 0x20, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73,
                    0x65, 0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
