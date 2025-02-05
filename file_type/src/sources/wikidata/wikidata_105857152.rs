use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857152: FileFormat = FileFormat {
    id: 105_857_152,
    source_type: SourceType::Wikidata,
    name: "MAME Hash",
    extensions: &["hsi"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
