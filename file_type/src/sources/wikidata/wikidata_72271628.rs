use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72271628: FileFormat = FileFormat {
    id: 72_271_628,
    source_type: SourceType::Wikidata,
    name: "ndr",
    extensions: &["ndr"],
    media_types: &["unknown"],
    internal_signatures: &[],
    related_formats: &[],
};
