use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52005965: FileFormat = FileFormat {
    id: 52_005_965,
    source_type: SourceType::Wikidata,
    name: "Micrografx Draw, version 3",
    extensions: &["drw"],
    media_types: &["application/x-mgx-designer"],
    signatures: &[],
    related_formats: &[],
};
