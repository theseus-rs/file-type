use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864137: FileFormat = FileFormat {
    id: 105_864_137,
    source_type: SourceType::Wikidata,
    name: "The Final Musicsystem eXtended module (pattern)",
    extensions: &["mdat"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x46, 0x4D, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
