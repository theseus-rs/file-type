use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206545: FileFormat = FileFormat {
    id: 28_206_545,
    source_type: SourceType::Wikidata,
    name: "MAKIchan Graphics MAG",
    extensions: &["mag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
