use crate::format::FileFormat;

pub(crate) const LINGUIST_7: FileFormat = FileFormat {
    id: 7,
    puid: "linguist/7",
    name: "ASN.1",
    extensions: &["asn", "asn1"],
    media_types: &["text/x-ttcn-asn"],
    internal_signatures: &[],
    related_formats: &[],
};
