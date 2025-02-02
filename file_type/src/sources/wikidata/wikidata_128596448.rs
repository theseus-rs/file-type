use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128596448: FileFormat = FileFormat {
    id: 128_596_448,
    source_type: SourceType::Wikidata,
    name: "Alloy file format",
    extensions: &["als"],
    media_types: &["text/x-alloy"],
    internal_signatures: &[],
    related_formats: &[],
};
