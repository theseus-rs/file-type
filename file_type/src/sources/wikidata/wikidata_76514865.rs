use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76514865: FileFormat = FileFormat {
    id: 76_514_865,
    source_type: SourceType::Wikidata,
    name: "WinDev Report",
    extensions: &["wde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
