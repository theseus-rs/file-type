use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119856975: FileFormat = FileFormat {
    id: 119_856_975,
    source_type: SourceType::Wikidata,
    name: "Streets & Trips File",
    extensions: &["est"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
