use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119856975: FileFormat = FileFormat {
    id: 119_856_975,
    source_type: SourceType::Wikidata,
    name: "Streets & Trips File",
    extensions: &["est"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
