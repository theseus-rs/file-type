use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826392: FileFormat = FileFormat {
    id: 27_826_392,
    source_type: SourceType::Wikidata,
    name: "Proxy Unrestricted Access Image",
    extensions: &["uai"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
