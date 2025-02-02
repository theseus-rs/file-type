use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975864: FileFormat = FileFormat {
    id: 28_975_864,
    source_type: SourceType::Wikidata,
    name: "OOGL Object File Format",
    extensions: &["off"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
