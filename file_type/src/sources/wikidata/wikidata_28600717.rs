use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600717: FileFormat = FileFormat {
    id: 28_600_717,
    source_type: SourceType::Wikidata,
    name: "DrawIt",
    extensions: &["drawit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
