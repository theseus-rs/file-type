use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72271628: FileFormat = FileFormat {
    id: 72_271_628,
    source_type: SourceType::Wikidata,
    name: "ndr",
    extensions: &["ndr"],
    media_types: &["unknown"],
    signatures: &[],
    related_formats: &[],
};
