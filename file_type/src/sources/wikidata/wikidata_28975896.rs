use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975896: FileFormat = FileFormat {
    id: 28_975_896,
    source_type: SourceType::Wikidata,
    name: "Spline Control File",
    extensions: &["spl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
