use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1027882: FileFormat = FileFormat {
    id: 1_027_882,
    puid: "wikidata/1027882",
    name: "Short Payment Descriptor",
    extensions: &["spayd"],
    media_types: &["application/x-shortpaymentdescriptor"],
    internal_signatures: &[],
    related_formats: &[],
};
