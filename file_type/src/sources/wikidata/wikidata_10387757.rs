use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_10387757: FileFormat = FileFormat {
    id: 10_387_757,
    source_type: SourceType::Wikidata,
    name: "Universal Image Format",
    extensions: &["uif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
