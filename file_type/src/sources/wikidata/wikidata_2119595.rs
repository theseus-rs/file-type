use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2119595: FileFormat = FileFormat {
    id: 2_119_595,
    source_type: SourceType::Wikidata,
    name: "Wavefront .obj file",
    extensions: &["object"],
    media_types: &["model/obj", "text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23])],
            },
        }],
    }],
    related_formats: &[],
};
