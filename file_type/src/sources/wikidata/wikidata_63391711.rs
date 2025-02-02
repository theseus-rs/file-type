use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63391711: FileFormat = FileFormat {
    id: 63_391_711,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Word Processor Macintosh, version 3",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
