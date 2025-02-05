use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206062: FileFormat = FileFormat {
    id: 28_206_062,
    source_type: SourceType::Wikidata,
    name: "Earth Resource Mapper Raster",
    extensions: &["ers"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
                    0x20, 0x42, 0x65, 0x67, 0x69, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
