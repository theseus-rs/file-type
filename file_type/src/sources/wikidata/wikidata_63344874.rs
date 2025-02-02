use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63344874: FileFormat = FileFormat {
    id: 63_344_874,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Word Processor 5-6",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
