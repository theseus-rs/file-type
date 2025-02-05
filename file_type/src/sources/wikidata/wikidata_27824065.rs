use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27824065: FileFormat = FileFormat {
    id: 27_824_065,
    source_type: SourceType::Wikidata,
    name: "Internet Archive ARC, version 1.1",
    extensions: &["arc"],
    media_types: &["application/x-internet-archive"],
    signatures: &[],
    related_formats: &[],
};
