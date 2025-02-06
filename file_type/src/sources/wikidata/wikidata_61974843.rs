use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61974843: FileFormat = FileFormat {
    id: 61_974_843,
    source_type: SourceType::Wikidata,
    name: "FoxPro Compound Index File",
    extensions: &["cdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
