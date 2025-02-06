use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3477565: FileFormat = FileFormat {
    id: 3_477_565,
    source_type: SourceType::Wikidata,
    name: "Secure Digital Container",
    extensions: &["sdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
