use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66244789: FileFormat = FileFormat {
    id: 66_244_789,
    source_type: SourceType::Wikidata,
    name: "ScreenCam stand-alone Movies format",
    extensions: &["exe"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
