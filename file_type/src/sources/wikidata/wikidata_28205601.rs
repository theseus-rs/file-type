use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205601: FileFormat = FileFormat {
    id: 28_205_601,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 1 Icon",
    extensions: &["icn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
