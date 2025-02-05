use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858967: FileFormat = FileFormat {
    id: 105_858_967,
    source_type: SourceType::Wikidata,
    name: "Blender physics external cache",
    extensions: &["bphys"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x50, 0x48, 0x59, 0x53, 0x49, 0x43, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
