use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111568387: FileFormat = FileFormat {
    id: 111_568_387,
    source_type: SourceType::Wikidata,
    name: "Managed Object Format",
    extensions: &["mof"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
