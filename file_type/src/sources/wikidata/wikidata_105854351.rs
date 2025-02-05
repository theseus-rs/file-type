use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854351: FileFormat = FileFormat {
    id: 105_854_351,
    source_type: SourceType::Wikidata,
    name: "Active WebCam Settings",
    extensions: &["awses"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4D, 0x61, 0x69, 0x6E, 0x5D, 0x0D, 0x0A, 0x4E, 0x75, 0x6D, 0x62, 0x65,
                    0x72, 0x4F, 0x66, 0x43, 0x61, 0x6D, 0x65, 0x72, 0x61, 0x73, 0x4F, 0x6E, 0x44,
                    0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
