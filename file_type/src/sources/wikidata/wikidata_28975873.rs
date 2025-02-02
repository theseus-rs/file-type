use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975873: FileFormat = FileFormat {
    id: 28_975_873,
    source_type: SourceType::Wikidata,
    name: "OOGL LIST file",
    extensions: &["list"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
