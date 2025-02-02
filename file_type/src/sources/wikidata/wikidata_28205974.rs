use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205974: FileFormat = FileFormat {
    id: 28_205_974,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Y Luminance Channel",
    extensions: &["imy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
