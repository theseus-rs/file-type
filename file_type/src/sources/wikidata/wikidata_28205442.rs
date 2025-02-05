use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205442: FileFormat = FileFormat {
    id: 28_205_442,
    source_type: SourceType::Wikidata,
    name: "GX2",
    extensions: &["gx2"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x58, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
