use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27894974: FileFormat = FileFormat {
    id: 27_894_974,
    source_type: SourceType::Wikidata,
    name: "Windows Media Redirector",
    extensions: &["wmx"],
    media_types: &["video/x-ms-wmx"],
    internal_signatures: &[],
    related_formats: &[],
};
