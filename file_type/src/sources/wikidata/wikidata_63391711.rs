use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63391711: FileFormat = FileFormat {
    id: 63_391_711,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Word Processor Macintosh, version 3",
    extensions: &["wps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
