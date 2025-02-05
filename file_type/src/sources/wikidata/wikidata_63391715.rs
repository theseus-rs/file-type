use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63391715: FileFormat = FileFormat {
    id: 63_391_715,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Word Processor Macintosh, version 4",
    extensions: &["wps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
