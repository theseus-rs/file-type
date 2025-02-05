use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63344874: FileFormat = FileFormat {
    id: 63_344_874,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Word Processor 5-6",
    extensions: &["wps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
