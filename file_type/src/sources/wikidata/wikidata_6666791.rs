use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6666791: FileFormat = FileFormat {
    id: 6_666_791,
    source_type: SourceType::Wikidata,
    name: "Log ASCII Standard Format",
    extensions: &["las"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
