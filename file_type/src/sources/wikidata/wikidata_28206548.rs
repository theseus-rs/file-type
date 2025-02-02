use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206548: FileFormat = FileFormat {
    id: 28_206_548,
    source_type: SourceType::Wikidata,
    name: "MAKIchan Graphics MAX",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
