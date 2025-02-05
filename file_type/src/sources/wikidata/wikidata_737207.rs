use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_737207: FileFormat = FileFormat {
    id: 737_207,
    source_type: SourceType::Wikidata,
    name: "RealVideo",
    extensions: &["rv"],
    media_types: &["application/vnd.rn-realmedia"],
    signatures: &[],
    related_formats: &[],
};
