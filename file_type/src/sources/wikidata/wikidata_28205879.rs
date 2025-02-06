use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205879: FileFormat = FileFormat {
    id: 28_205_879,
    source_type: SourceType::Wikidata,
    name: "CUT",
    extensions: &["cut"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
