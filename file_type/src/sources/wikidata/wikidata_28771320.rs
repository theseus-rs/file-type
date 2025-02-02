use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28771320: FileFormat = FileFormat {
    id: 28_771_320,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office File List",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
