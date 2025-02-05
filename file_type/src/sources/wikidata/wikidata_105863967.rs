use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863967: FileFormat = FileFormat {
    id: 105_863_967,
    source_type: SourceType::Wikidata,
    name: "Meshwork model (v1.0)",
    extensions: &["mesh"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x65, 0x73, 0x68, 0x09, 0x31, 0x09, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
