use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63106742: FileFormat = FileFormat {
    id: 63_106_742,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Word Processor file 3-4 for Windows",
    extensions: &["wps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
