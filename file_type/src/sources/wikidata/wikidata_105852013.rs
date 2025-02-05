use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852013: FileFormat = FileFormat {
    id: 105_852_013,
    source_type: SourceType::Wikidata,
    name: "Csound Score",
    extensions: &["sco"],
    media_types: &["audio/csound"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0F, 0x43, 0x43, 0x53, 0x43, 0x4F, 0x52, 0x43, 0x48, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
