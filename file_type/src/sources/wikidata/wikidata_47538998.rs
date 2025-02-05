use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47538998: FileFormat = FileFormat {
    id: 47_538_998,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Landscape Library",
    extensions: &["lli"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
