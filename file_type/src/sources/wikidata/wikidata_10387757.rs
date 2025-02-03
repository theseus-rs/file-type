use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10387757: FileFormat = FileFormat {
    id: 10_387_757,
    source_type: SourceType::Wikidata,
    name: "Universal Image Format",
    extensions: &["uif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
