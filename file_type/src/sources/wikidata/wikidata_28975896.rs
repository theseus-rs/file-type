use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975896: FileFormat = FileFormat {
    id: 28_975_896,
    source_type: SourceType::Wikidata,
    name: "Spline Control File",
    extensions: &["spl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
