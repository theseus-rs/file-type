use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863676: FileFormat = FileFormat {
    id: 105_863_676,
    source_type: SourceType::Wikidata,
    name: "MacWrite II document",
    extensions: &["mcw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x2E, 0x00, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
