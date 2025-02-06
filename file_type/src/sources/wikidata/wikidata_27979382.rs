use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979382: FileFormat = FileFormat {
    id: 27_979_382,
    source_type: SourceType::Wikidata,
    name: "MPLS",
    extensions: &["mpl", "mpls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
