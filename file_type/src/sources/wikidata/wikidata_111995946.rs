use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111995946: FileFormat = FileFormat {
    id: 111_995_946,
    source_type: SourceType::Wikidata,
    name: "Microsoft Paint File",
    extensions: &["msp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
