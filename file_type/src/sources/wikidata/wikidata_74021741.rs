use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74021741: FileFormat = FileFormat {
    id: 74_021_741,
    source_type: SourceType::Wikidata,
    name: "Microsoft Resource Library",
    extensions: &["rll"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
