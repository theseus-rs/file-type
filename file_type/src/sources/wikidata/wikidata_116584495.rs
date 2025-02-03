use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116584495: FileFormat = FileFormat {
    id: 116_584_495,
    source_type: SourceType::Wikidata,
    name: "Microsoft Windows Snipping Tool Snip Data",
    extensions: &["snip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
