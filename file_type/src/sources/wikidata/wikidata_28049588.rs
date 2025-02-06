use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049588: FileFormat = FileFormat {
    id: 28_049_588,
    source_type: SourceType::Wikidata,
    name: "Tiny Stuff, low resolution",
    extensions: &["tn1", "tny"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
