use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205611: FileFormat = FileFormat {
    id: 28_205_611,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 1 Hot Icon",
    extensions: &["hot"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
