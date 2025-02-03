use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1027882: FileFormat = FileFormat {
    id: 1_027_882,
    source_type: SourceType::Wikidata,
    name: "Short Payment Descriptor",
    extensions: &["spayd"],
    media_types: &["application/x-shortpaymentdescriptor"],
    internal_signatures: &[],
    related_formats: &[],
};
