use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863629: FileFormat = FileFormat {
    id: 105_863_629,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD Mesh (v3.0)",
    extensions: &["msh"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x4D, 0x53, 0x48, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
