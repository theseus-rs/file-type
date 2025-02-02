use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51841970: FileFormat = FileFormat {
    id: 51_841_970,
    source_type: SourceType::Wikidata,
    name: "Microsoft FoxPro Library",
    extensions: &["plb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
