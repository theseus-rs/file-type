use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975617: FileFormat = FileFormat {
    id: 28_975_617,
    source_type: SourceType::Wikidata,
    name: "GNU Triangulated Surface",
    extensions: &["gts"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
