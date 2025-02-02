use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975647: FileFormat = FileFormat {
    id: 28_975_647,
    source_type: SourceType::Wikidata,
    name: "POV-Ray RAW triangle format",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
